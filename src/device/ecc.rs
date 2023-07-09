use crate::{device::DeviceArgs, DecodeError, Result};
use helium_crypto::{
    ecc608::{self, with_ecc, Ecc},
    Keypair, Network,
};
use http::Uri;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Device {
    /// The i2c/swi device path
    pub path: PathBuf,
    /// The bus address
    pub address: u16,
    /// The ecc slot to use
    pub slot: u8,
}

impl Device {
    /// Parses an ecc device url of the form `ecc:<dev>[:address][?slot=<slot>]`,
    /// where <dev> is the device file name (usually begins with i2c or tty),
    /// <address> is the bus address (default 96, ignored for swi), and <slot>
    /// is the slot to use for key lookup/manipulation (default: 0)
    pub fn from_url(url: &Uri) -> Result<Self> {
        let args = DeviceArgs::from_uri(url)?;
        let address = url.port_u16().unwrap_or(96);
        let slot = args.get("slot", 0)?;
        let path = url
            .host()
            .map(|dev| Path::new("/dev").join(dev))
            .ok_or_else(|| DecodeError::InvalidDeviceUrl(url.to_string()))?;

        Ok(Self {
            path,
            address,
            slot,
        })
    }

    pub fn get_info(&self) -> Result<Info> {
        Ok(Info {
            path: self.path.clone(),
            address: self.address,
            slot: self.slot,
        })
    }

    pub fn get_keypair(&self, create: bool) -> Result<Keypair> {
        let keypair: Keypair = with_ecc(|ecc| {
            if create {
                generate_compact_key_in_slot(ecc, self.slot)
            } else {
                compact_key_in_slot(ecc, self.slot)
            }
        })?;
        Ok(keypair)
    }
}

#[derive(Debug, Serialize)]
pub struct Info {
    /// The i2c/swi device path
    pub path: PathBuf,
    /// The bus address
    pub address: u16,
    /// The ecc slot to use
    pub slot: u8,
}

fn compact_key_in_slot(ecc: &mut Ecc, slot: u8) -> Result<Keypair> {
    let keypair = ecc608::Keypair::from_ecc_slot(ecc, Network::MainNet, slot)?;
    Ok(keypair.into())
}

fn generate_compact_key_in_slot(ecc: &mut Ecc, slot: u8) -> Result<Keypair> {
    let mut try_count = 5;
    loop {
        ecc.genkey(ecc608::KeyType::Private, slot)
            .map_err(helium_crypto::Error::from)?;

        match compact_key_in_slot(ecc, slot) {
            Ok(keypair) => return Ok(keypair),
            Err(err) if try_count == 0 => return Err(err),
            Err(_) => try_count -= 1,
        }
    }
}
