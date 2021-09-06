use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Q3Error {
    /// An error during parsing data
    #[error("parse error {0}")]
    ParseError(#[from] ParseIntError),

    /// An error during UDP socket operations
    #[error("udp error {0}")]
    UDPError(#[from] std::io::Error),
}
