use core::fmt;

use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const PACKET_END: u8 = 0;
pub trait Encode {
    fn encode(&self) -> Vec<u8>
    where
        Self: Serialize,
    {
        let ser = bincode::serialize(&self).unwrap();
        BASE64_STANDARD
            .encode(ser)
            .as_bytes()
            .iter()
            .chain(std::iter::once(&PACKET_END))
            .copied()
            .collect()
    }
}

pub trait Decode<T>
where
    T: DeserializeOwned,
{
    fn decode(bytes: &[u8]) -> T {
        let x = BASE64_STANDARD.decode(bytes).unwrap();
        let des: T = bincode::deserialize(&x).unwrap();
        des
    }
}

#[derive(Serialize, Deserialize)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Encode for Direction {}
impl Decode<Direction> for Direction {}

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

impl Encode for Command {}
impl Decode<Command> for Command {}

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

impl Encode for Request {}
impl Decode<Request> for Request {}

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
