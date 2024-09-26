use anyhow::Result;
use clap::Parser;
mod commands;
mod rpc;

pub const DEFAULT_RPC_URL: &str = "https://mainnet.era.zksync.io";

#[derive(Parser)]
#[command(name = "zeek")]
#[command(about = "CLI tool for ZKsync", long_about = None)]
struct Cli {
    /// The RPC URL to use (defaults to https://mainnet.era.zksync.io)
    #[arg(long, global = true, default_value = DEFAULT_RPC_URL)]
    rpc_url: String,

    #[command(subcommand)]
    command: commands::Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    // Handle the command for Proof
    match &cli.command {
        commands::Commands::Gas(gas_cmd) => {
            commands::gas::handle_gas_command(gas_cmd, &cli.rpc_url).await?;
        }
    }

    Ok(())
}
