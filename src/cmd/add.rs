use crate::{cmd::print_json, txn_sign::TxnSign};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use gateway_security_rs::device::Device;
use helium_proto::{BlockchainTxn, BlockchainTxnAddGatewayV1, Message, Txn};
use serde_json::json;

/// Construct an add gateway transaction for this gateway.
#[derive(Debug, clap::Args)]
pub struct Cmd {
    /// The owner to use in the generated add transaction. This is a helium
    /// public key in string form.
    owner: helium_crypto::PublicKey,
}

impl Cmd {
    pub fn run(&self, device: &Device) -> anyhow::Result<()> {
        let keypair = device.get_keypair(false)?;
        let mut txn = BlockchainTxnAddGatewayV1 {
            gateway: keypair.public_key().to_vec(),
            owner: self.owner.to_vec(),
            payer: vec![],
            fee: 0,
            staking_fee: 0,
            owner_signature: vec![],
            gateway_signature: vec![],
            payer_signature: vec![],
        };

        txn.gateway_signature = txn.sign(&keypair)?;

        let add_gateway_txn = BlockchainTxn {
            txn: Some(Txn::AddGateway(txn)),
        }
        .encode_to_vec();

        let encoded_txn = B64.encode(add_gateway_txn);
        let json = json!({
            "address": keypair.public_key().to_string(),
            "txn": encoded_txn,
        });
        print_json(&json)
    }
}
