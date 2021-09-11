use std::{num::ParseIntError, str::Utf8Error};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Q3Error {
    /// An error while parsing score/ping from `str` to `i32`
    #[error("parse error {0}")]
    ParseError(#[from] ParseIntError),

    /// An error during UDP socket operations
    #[error("udp error {0}")]
    UDPError(#[from] std::io::Error),

    /// An error made by the caller
    #[error("user error")]
    UserError,

    /// An error parsing bytes as UTF-8
    #[error("utf-8 error {0}")]
    Utf8Error(#[from] Utf8Error),
}
