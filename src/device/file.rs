use crate::Result;
use helium_crypto::{KeyTag, Keypair};
use http::Uri;
use rand::rngs::OsRng;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Device {
    /// The file device path
    pub path: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct Info {
    r#type: String,
    path: PathBuf,
}

impl Device {
    /// Parses a file device url of the form `file://<path>`,
    pub fn from_url(url: &Uri) -> Result<Self> {
        Ok(Self {
            path: url.path().into(),
        })
    }

    pub fn get_info(&self) -> Result<Info> {
        let keypair = self.get_keypair(false)?;
        let key_type = keypair.key_tag().key_type.to_string();
        let info = Info {
            r#type: key_type,
            path: self.path.clone(),
        };
        Ok(info)
    }

    pub fn get_keypair(&self, create: bool) -> Result<Keypair> {
        if !self.path.exists() || create {
            let keypair = Keypair::generate(KeyTag::default(), &mut OsRng);
            fs::write(&self.path, keypair.to_vec())?;
        }
        load_keypair(&self.path)
    }
}

fn load_keypair<P: AsRef<Path>>(path: &P) -> Result<Keypair> {
    let data = fs::read(path)?;
    let keypair = Keypair::try_from(&data[..])?;
    Ok(keypair)
}
