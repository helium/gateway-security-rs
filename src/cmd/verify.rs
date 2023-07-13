use crate::{cmd::print_json, txn_sign::TxnSign};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use gateway_security_rs::device::Device;
use helium_proto::{BlockchainTxn, Message, Txn};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct Transaction(BlockchainTxn);

impl std::str::FromStr for Transaction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let decoded = B64.decode(s)?;
        Ok(Self(BlockchainTxn::decode(decoded.as_ref())?))
    }
}

/// Construct an add gateway transaction for this gateway.
#[derive(Debug, clap::Args)]
pub struct Cmd {
    /// The transaction to verify
    txn: Transaction,
}

impl Cmd {
    pub fn run(&self, _device: &Device) -> anyhow::Result<()> {
        match &self.txn.0.txn {
            Some(Txn::AddGateway(txn)) => {
                let gateway = helium_crypto::PublicKey::try_from(txn.gateway.as_ref())?;
                let json = json!({
                    "address": gateway.to_string(),
                    "owner": helium_crypto::PublicKey::try_from(txn.owner.as_ref())?,
                    "verify": txn.verify(&gateway, &txn.gateway_signature).is_ok(),
                });
                print_json(&json)
            }
            _ => anyhow::bail!("invalid transation type"),
        }
    }
}
