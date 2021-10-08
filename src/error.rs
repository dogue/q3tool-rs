use core::fmt;
use std::{error::Error, fmt::Display, io, num::ParseIntError, str::Utf8Error};

#[derive(Debug)]
pub enum Q3Error {
    /// Occurs when parsing either a players ping or score from a string to an int
    ParseError(ParseIntError),
    /// Any network error that may occur while creating, sending, or receiving with the UDP socket
    UdpError(io::Error),
    /// Occurs when converting the `&[u8]` buffer into a String
    Utf8Error(Utf8Error),
    /// Occurs when the response from the server is empty or otherwise malformed and cannot be parsed properly
    InvalidResponse,
}

impl Display for Q3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Q3Error::ParseError(e) => write!(
                f,
                "failed to parse int from score or ping: {}",
                e.to_string()
            ),
            Q3Error::UdpError(e) => write!(f, "socket operation failed: {}", e.to_string()),
            Q3Error::Utf8Error(e) => write!(f, "invalid utf-8 byte: {}", e.to_string()),
            Q3Error::InvalidResponse => {
                write!(f, "response from server was empty or improperly formatted")
            }
        }
    }
}

impl Error for Q3Error {}

impl From<ParseIntError> for Q3Error {
    fn from(error: ParseIntError) -> Self {
        Q3Error::ParseError(error)
    }
}

impl From<io::Error> for Q3Error {
    fn from(error: io::Error) -> Self {
        Q3Error::UdpError(error)
    }
}

impl From<Utf8Error> for Q3Error {
    fn from(error: Utf8Error) -> Self {
        Q3Error::Utf8Error(error)
    }
}
