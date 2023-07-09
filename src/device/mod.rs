use crate::result::{DecodeError, Error, Result};
use http::Uri;
use serde::Serialize;
use std::collections::HashMap;

#[cfg(feature = "ecc608")]
pub mod ecc;
pub mod file;
#[cfg(feature = "nova-tz")]
pub mod nova_tz;
#[cfg(feature = "tpm")]
pub mod tpm;

/// Device arguments are key/value pairs that can be passed to a device. They
/// are usually URL encoded in the Device URL.
pub struct DeviceArgs(HashMap<String, String>);

/// A security device to work with. Security devices come in all forms. This
/// abstracts them into one with a well defined interface for doing what this
/// tool needs to do with them.
#[derive(Debug, Clone)]
pub enum Device {
    #[cfg(feature = "ecc608")]
    Ecc(ecc::Device),
    #[cfg(feature = "tpm")]
    Tpm(tpm::Device),
    #[cfg(feature = "nova-tz")]
    TrustZone(nova_tz::Device),
    File(file::Device),
}

/// Represents useful device information like model and serial number.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Info {
    #[cfg(feature = "ecc608")]
    Ecc(ecc::Info),
    #[cfg(feature = "tpm")]
    Tpm(tpm::Info),
    #[cfg(feature = "nova-tz")]
    TrustZone(nova_tz::Info),
    File(file::Info),
}

impl DeviceArgs {
    pub fn from_uri(url: &Uri) -> Result<Self> {
        let args = url
            .query()
            .map_or_else(
                || Ok(HashMap::new()),
                serde_urlencoded::from_str::<HashMap<String, String>>,
            )
            .map_err(|_| DecodeError::InvalidDeviceUrl(url.to_string()))?;
        Ok(Self(args))
    }

    pub fn get<T>(&self, name: &str, default: T) -> Result<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let parsed = self
            .0
            .get(name)
            .map(|s| s.parse::<T>())
            .unwrap_or(Ok(default))
            .map_err(|_err| DecodeError::InvalidDeviceUrlArgument(name.to_string()))?;
        Ok(parsed)
    }
}

impl Device {
    pub fn get_keypair(&self, create: bool) -> Result<helium_crypto::Keypair> {
        let keypair = match self {
            #[cfg(feature = "ecc608")]
            Self::Ecc(device) => device.get_keypair(create)?,
            #[cfg(feature = "tpm")]
            Self::Tpm(device) => device.get_keypair(create)?,
            #[cfg(feature = "nova-tz")]
            Self::TrustZone(device) => device.get_keypair(create)?,
            Self::File(device) => device.get_keypair(create)?,
        };
        Ok(keypair)
    }

    pub fn get_info(&self) -> crate::Result<Info> {
        let info = match self {
            #[cfg(feature = "ecc608")]
            Self::Ecc(device) => Info::Ecc(device.get_info()?),
            #[cfg(feature = "tpm")]
            Self::Tpm(device) => Info::Tpm(device.get_info()?),
            #[cfg(feature = "nova-tz")]
            Self::TrustZone(device) => Info::TrustZone(device.get_info()?),
            Self::File(device) => Info::File(device.get_info()?),
        };
        Ok(info)
    }
}

impl std::str::FromStr for Device {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let url: http::Uri = s.parse()?;
        match url.scheme_str() {
            #[cfg(feature = "ecc608")]
            Some("ecc") => Ok(Self::Ecc(ecc::Device::from_url(&url)?)),
            #[cfg(feature = "tpm")]
            Some("tpm") => Ok(Self::Tpm(tpm::Device::from_url(&url)?)),
            #[cfg(feature = "nova-tz")]
            Some("nova-tz") => Ok(Self::TrustZone(nova_tz::Device::from_url(&url)?)),
            Some("file") | None => Ok(Self::File(file::Device::from_url(&url)?)),
            _ => Err(DecodeError::InvalidDeviceUrl(url.to_string()).into()),
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

    #[cfg(feature = "nova-tz")]
    #[test]
    fn test_device_nova_tz() {
        let device: Device = "nova-tz://rsa/tmp/rsa_key_blob"
            .parse()
            .expect("nova-tz device");
        assert!(matches!(device, Device::TrustZone(_)));
    }

    #[cfg(feature = "ecc608")]
    #[test]
    fn test_device_ecc608() {
        let device: Device = "ecc://i2c-1:96?slot=0".parse().expect("ecc device");
        assert!(matches!(device, Device::Ecc(_)));
    }
}
