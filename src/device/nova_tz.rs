use crate::Result;
use helium_crypto::{nova_tz, Keypair, Network};
use http::Uri;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Device {
    /// TrustZone keyblob path
    pub path: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct Info {
    path: PathBuf,
}

impl Device {
    /// Parses a trustzone device url of the form `nova-tz://rsa/<key_path>`,
    /// where <key_path> is the path to TrustZone keyblob
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
            panic!("not supported")
        }

        let keypair = nova_tz::Keypair::from_key_path(Network::MainNet, &self.path)
            .map(helium_crypto::Keypair::from)?;
        Ok(keypair)
    }
}
