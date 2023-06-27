use crate::result::{DecodeError, Error, Result};
use serde::Serialize;

pub mod file;

/// A security device to work with. Security devices come in all forms. This
/// abstracts them into one with a well defined interface for doing what this
/// tool needs to do with them.
#[derive(Debug, Clone)]
pub enum Device {
    File(file::Device),
}

/// Represents useful device information like model and serial number.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Info {
    File(file::Info),
}

impl Device {
    pub fn get_keypair(&self, create: bool) -> crate::Result<helium_crypto::Keypair> {
        let keypair = match self {
            Self::File(device) => device.get_keypair(create)?,
        };
        Ok(keypair)
    }

    pub fn get_info(&self) -> crate::Result<Info> {
        let info = match self {
            Self::File(device) => Info::File(device.get_info()?),
        };
        Ok(info)
    }
}

impl std::str::FromStr for Device {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let url: http::Uri = s.parse().map_err(Error::from)?;
        match url.scheme_str() {
            Some("file") | None => Ok(Self::File(file::Device::from_url(&url)?)),
            _ => Err(DecodeError::InvalidUriScheme(url.to_string()).into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_device_file() {
        let device: Device = "/tmp/keypair.bin".parse().expect("file device");
        assert!(matches!(device, Device::File(_)));
    }
}
