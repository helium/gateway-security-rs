use crate::cmd::print_json;
use gateway_onboarding_rs::device::Device;
use serde_json::json;

/// Construct an add gateway transaction for this gateway.
#[derive(Debug, clap::Args)]
pub struct Cmd {}

impl Cmd {
    pub fn run(&self, device: &Device) -> anyhow::Result<()> {
        let keypair = device.get_keypair(false)?;
        let json = json!({
            "public_key": keypair.public_key().to_string(),
            "info": device.get_info()?,
        });
        print_json(&json)
    }
}
