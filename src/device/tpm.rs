use crate::{Error, Result};
use helium_crypto::{tpm, Keypair, Network};
use http::Uri;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Device {
    /// TPM key path
    pub path: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct Info {
    path: PathBuf,
}

impl Device {
    /// Parses a tpm device url of the form `tpm://tpm/<key_path>`,
    /// where <key_path> is the path to TPM KEY
    pub fn from_url(url: &Uri) -> Result<Self> {
        Ok(Self {
            path: url.path().into(),
        })
    }

    pub fn get_info(&self) -> Result<Info> {
        Ok(Info {
            path: self.path.clone(),
        })
    }

    pub fn get_keypair(&self, create: bool) -> Result<Keypair> {
        if create {
            return Err(Error::CreateNotSupported);
        }
        let keypair = tpm::Keypair::from_key_path(Network::MainNet, &self.path.to_string_lossy())
            .map(helium_crypto::Keypair::from)?;
        Ok(keypair)
    }
}
