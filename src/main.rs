use clap::Parser;
use gateway_security_rs::device::Device;

mod cmd;
mod txn_sign;

#[derive(Debug, Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(name = env!("CARGO_BIN_NAME"))]
/// Helium Gateway
pub struct Cli {
    /// The security device to use.
    ///
    /// The URL for the security device is dependent on the device type being
    /// used.
    ///
    /// Examples:
    ///
    /// ecc608 - "ecc://i2c-1", "ecc://i2c-1:96?slot=0"
    /// file - "file:///etc/keypair.bin"\n
    /// tpm - "tpm://tpm/<key_path>"
    #[arg(long, verbatim_doc_comment)]
    device: Device,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Subcommand)]
pub enum Cmd {
    Info(cmd::info::Cmd),
    Add(cmd::add::Cmd),
    Verify(cmd::verify::Cmd),
}

pub fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.cmd.run(&cli.device)
}

impl Cmd {
    fn run(&self, device: &Device) -> anyhow::Result<()> {
        match self {
            Self::Info(cmd) => cmd.run(device),
            Self::Add(cmd) => cmd.run(device),
            Self::Verify(cmd) => cmd.run(device),
        }
    }
}
