use crate::network::command::data_wrapper::DataWrapper;
use crate::network::command::echo::EchoRequest;
use crate::network::command::nil::NilRequest;
use crate::network::command::ping::PingRequest;
use crate::network::command::Command;
use crate::network::command::set::SetRequest;
use bytes::{Bytes, BytesMut};
use memchr::memchr;
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::str::from_utf8;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Decoder;
use crate::network::command::get::GetRequest;
use crate::storage::MemoryStorage;

fn match_command(string: Bytes, db: Arc<MemoryStorage>) -> Option<Box<dyn Command>> {
    match string.iter().as_slice() {
        b"PING" => Some(Box::new(PingRequest)),
        b"ECHO" => Some(Box::new(EchoRequest::new())),
        b"SET" => Some(Box::new(SetRequest::new(db))),
        b"GET" => Some(Box::new(GetRequest::new(db))),
        _ => None,
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum RedisValueRef {
    String(Bytes),
    Error(Bytes),
    Int(i64),
    Array(Vec<RedisValueRef>),
    NullArray,
    NullBulkString,
    ErrorMsg(Vec<u8>),
}

impl RedisValueRef {
    fn get_command(self, db: Arc<MemoryStorage>) -> Vec<Option<Box<dyn Command>>> {
        match self {
            RedisValueRef::String(string) => {
                Vec::from([
                    match_command(string.clone(), db)
                    .or(Some(Box::new(DataWrapper::new(string.clone()))))
                ])
            }
            RedisValueRef::Error(string) => Vec::from([match_command(string, db)]),
            RedisValueRef::Array(vec) => {
                vec.into_iter().map(|c| c.get_command(db.clone())).flatten().collect()
            }
            _ => Vec::new(),
        }
    }

    fn get_bytes(self) -> Bytes {
        match self {
            RedisValueRef::String(bytes) => bytes,
            _ => Bytes::new(),
        }
    }
}

struct BufSplit(usize, usize);

// First, we need a convenient way to convert our index pairs into byte slices.
impl BufSplit {
    /// Get a lifetime appropriate slice of the underlying buffer.
    ///
    /// Constant time.
    #[inline]
    fn as_slice<'a>(&self, buf: &'a BytesMut) -> &'a [u8] {
        &buf[self.0..self.1]
    }

    /// Get a Bytes object representing the appropriate slice
    /// of bytes.
    ///
    /// Constant time.
    #[inline]
    fn as_bytes(&self, buf: &Bytes) -> Bytes {
        buf.slice(self.0..self.1)
    }
}

enum RedisBufSplit {
    String(BufSplit),
    Error(BufSplit),
    Int(i64),
    Array(Vec<RedisBufSplit>),
    NullArray,
    NullBulkString,
}

impl RedisBufSplit {
    fn redis_value(self, buf: &Bytes) -> RedisValueRef {
        match self {
            // bfs is BufSplit(start, end), which has the as_bytes method defined above
            RedisBufSplit::String(bfs) => RedisValueRef::String(bfs.as_bytes(buf)),
            RedisBufSplit::Error(bfs) => RedisValueRef::Error(bfs.as_bytes(buf)),
            RedisBufSplit::Array(arr) => {
                RedisValueRef::Array(arr.into_iter().map(|bfs| bfs.redis_value(buf)).collect())
            }
            RedisBufSplit::NullArray => RedisValueRef::NullArray,
            RedisBufSplit::NullBulkString => RedisValueRef::NullBulkString,
            RedisBufSplit::Int(i) => RedisValueRef::Int(i),
        }
    }
}

#[derive(Debug)]
pub enum RESPError {
    UnexpectedEnd,
    UnknownStartingByte,
    IOError(std::io::Error),
    IntParseFailure,
    BadBulkStringSize(i64),
    BadArraySize(i64),
}

impl From<std::io::Error> for RESPError {
    fn from(_err: std::io::Error) -> RESPError {
        RESPError::IOError(_err)
    }
}

type RedisResult = Result<Option<(usize, RedisBufSplit)>, RESPError>;

#[inline]
fn word(buf: &BytesMut, pos: usize) -> Option<(usize, BufSplit)> {
    // We're at the edge of `buf`, so we can't find a word.
    if buf.len() <= pos {
        return None;
    }
    // Find the position of the b'\r'
    memchr(b'\r', &buf[pos..]).and_then(|end| {
        if end + 1 < buf.len() {
            // pos + end == first index of b'\r' after `pos`
            // pos + end + 2 == ..word\r\n<HERE> -- skip to after CLRF
            Some((pos + end + 2, BufSplit(pos, pos + end)))
        } else {
            // Edge case: We received just enough bytes from the client
            // to get the \r but not the \n
            None
        }
    })
}

fn simple_string(buf: &BytesMut, pos: usize) -> RedisResult {
    Ok(word(buf, pos).map(|(pos, word)| (pos, RedisBufSplit::String(word))))
}

fn error(buf: &BytesMut, pos: usize) -> RedisResult {
    Ok(word(buf, pos).map(|(pos, word)| (pos, RedisBufSplit::Error(word))))
}

fn int(buf: &BytesMut, pos: usize) -> Result<Option<(usize, i64)>, RESPError> {
    match word(buf, pos) {
        Some((pos, word)) => {
            // word.as_slice(buf) is the method call BufSplit::as_slice(&self, &BytesMut) to access the byte slice.
            let s = from_utf8(word.as_slice(buf)).map_err(|_| RESPError::IntParseFailure)?;
            // Convert the string to an i64. Note the `?` for early returns.
            let i = s.parse().map_err(|_| RESPError::IntParseFailure)?;
            Ok(Some((pos, i)))
        }
        None => Ok(None),
    }
}

fn resp_int(buf: &BytesMut, pos: usize) -> RedisResult {
    Ok(int(buf, pos)?.map(|(pos, int)| (pos, RedisBufSplit::Int(int))))
}

fn bulk_string(buf: &BytesMut, pos: usize) -> RedisResult {
    match int(buf, pos)? {
        Some((pos, -1)) => Ok(Some((pos, RedisBufSplit::NullBulkString))),
        Some((pos, size)) if size >= 0 => {
            let total_size = pos + size as usize;
            if buf.len() < total_size + 2 {
                Ok(None)
            } else {
                let bb = RedisBufSplit::String(BufSplit(pos, total_size));
                Ok(Some((total_size + 2, bb)))
            }
        }
        Some((_pos, bad_size)) => Err(RESPError::BadBulkStringSize(bad_size)),
        None => Ok(None),
    }
}

fn array(buf: &BytesMut, pos: usize) -> RedisResult {
    match int(buf, pos)? {
        None => Ok(None),
        Some((pos, -1)) => Ok(Some((pos, RedisBufSplit::NullArray))),
        Some((pos, num_elements)) if num_elements >= 0 => {
            let mut values = Vec::with_capacity(num_elements as usize);
            let mut curr_pos = pos;
            for _ in 0..num_elements {
                match parse(buf, curr_pos)? {
                    Some((new_pos, value)) => {
                        curr_pos = new_pos;
                        values.push(value);
                    }
                    None => return Ok(None),
                }
            }
            Ok(Some((curr_pos, RedisBufSplit::Array(values))))
        }
        Some((_pos, bad_num_elements)) => Err(RESPError::BadArraySize(bad_num_elements)),
    }
}

fn parse(buf: &BytesMut, pos: usize) -> RedisResult {
    if buf.is_empty() {
        return Ok(None);
    }

    match buf[pos] {
        b'+' => simple_string(buf, pos + 1),
        b'-' => error(buf, pos + 1),
        b'$' => bulk_string(buf, pos + 1),
        b':' => resp_int(buf, pos + 1),
        b'*' => array(buf, pos + 1),
        _ => Err(RESPError::UnknownStartingByte),
    }
}

#[derive(Default)]
pub struct RespParser;

impl Decoder for RespParser {
    type Item = RedisValueRef;
    type Error = RESPError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if buf.is_empty() {
            return Ok(None);
        }

        match parse(buf, 0)? {
            Some((pos, value)) => {
                // We parsed a value! Shave off the bytes so tokio can continue filling the buffer.
                let our_data = buf.split_to(pos);
                // Use `redis_value` defined above to get the correct type
                Ok(Some(value.redis_value(&our_data.freeze())))
            }
            None => Ok(None),
        }
    }
}

pub struct ConnectionManager {
    socket: TcpStream,
}

impl ConnectionManager {
    pub fn new(socket: TcpStream) -> ConnectionManager {
        ConnectionManager { socket }
    }

    //old impl
    pub async fn listen(self, db: Arc<MemoryStorage>) {
        let mut transport = RespParser::default().framed(self.socket);

        while let Some(redis_value) = transport.next().await {
            println!("Readed command: {:?}", redis_value);

            let transport = transport.get_mut();
            let mut last_command: Box<dyn Command> = Box::new(NilRequest);

            match redis_value {
                Ok(rv) => {
                    for command in rv.get_command(db.clone()) {
                        match command {
                            Some(cmd) => {

                                println!("Readed actual-command: {:?}", cmd);
                                
                                if TypeId::of::<DataWrapper>() != (*cmd).type_id() {
                                    last_command = cmd;
                                } else {
                                    last_command.set_data(cmd.get_data());
                                }
                            }
                            None => {}
                        }

                        if last_command.needs_more_reading() {
                            continue;
                        }

                        transport
                            .write_all(last_command.process().as_bytes())
                            .await
                            .expect("Error writing response");
                    }
                }
                Err(_) => println!("Error getting redis command"),
            }
        }
    }
}
