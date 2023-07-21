use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not find signature")]
    SignatureNotFoundError,

    #[error("could not find netvar")]
    NetvarNotFoundError,

    #[error("unknown io error")]
    IoError(#[from] std::io::Error),

    #[error("failure when converting from utf8")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("failure when parsing toml")]
    ParseError(#[from] toml::de::Error),

    #[error("reqwest failure")]
    ReqwestError(#[from] reqwest::Error),

    #[error("unknown memflow error")]
    MemflowError(#[from] memflow::error::Error),

    #[error("unrar error")]
    MemflowPartial { msg: String },

    #[error("unknown data store error")]
    Unknown,
}

impl<T> From<memflow::error::PartialError<T>> for Error {
    fn from(err: memflow::error::PartialError<T>) -> Self {
        let msg = err.to_string();
        Self::MemflowPartial { msg }
    }
}
