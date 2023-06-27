use thiserror::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("encode error: {0}")]
    Encode(#[from] EncodeError),
    #[error("encode error: {0}")]
    Decode(#[from] DecodeError),
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("crypto error: {0}")]
    CryptoError(#[from] helium_crypto::Error),
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("protobuf encode")]
    Prost(#[from] prost::EncodeError),
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("invalid device url: \"{0}\"")]
    IvalidDeviceUrl(#[from] http::uri::InvalidUri),
    #[error("invalid device scheme: \"{0}\"")]
    InvalidUriScheme(String),
}

macro_rules! from_err {
    ($to_type:ty, $from_type:ty) => {
        impl From<$from_type> for Error {
            fn from(v: $from_type) -> Self {
                Self::from(<$to_type>::from(v))
            }
        }
    };
}

// Encode Errors
from_err!(EncodeError, prost::EncodeError);

// Decode Errors
from_err!(DecodeError, http::uri::InvalidUri);
