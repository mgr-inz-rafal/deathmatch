use core::fmt;

use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};

use tokio_util::{
    bytes::{self, Buf},
    codec,
};

pub const PACKET_END: u8 = 0;

pub struct RequestCodec {}

impl codec::Decoder for RequestCodec {
    type Item = Request;

    type Error = std::io::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let len = src.len();
        for i in 0..len {
            if src[i] == PACKET_END {
                // We have full frame.
                let data = &src[0..i];
                let x = BASE64_STANDARD.decode(data).unwrap();
                let des: Self::Item = bincode::deserialize(&x).unwrap();
                src.advance(i + 1);
                return Ok(Some(des));
            }
        }
        Ok(None)
    }
}

impl codec::Encoder<Request> for RequestCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Request, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let ser = bincode::serialize(&item).unwrap();
        dst.extend(
            BASE64_STANDARD
                .encode(ser)
                .as_bytes()
                .iter()
                .chain(std::iter::once(&PACKET_END)),
        );
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Command {
    Quit,
    Move(Direction),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Quit => write!(f, "Quit"),
            Command::Move(direction) => write!(f, "Move({})", direction),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    command: Command,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request({})", self.command)
    }
}

impl Request {
    pub fn left() -> Self {
        Self {
            command: Command::Move(Direction::Left),
        }
    }

    pub fn right() -> Self {
        Self {
            command: Command::Move(Direction::Right),
        }
    }

    pub fn quit() -> Self {
        Self {
            command: Command::Quit,
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Code {
    Ok,
    Error,
}

#[derive(Serialize, Deserialize)]
struct Response {
    code: Code,
}

#[cfg(test)]
mod tests {}
