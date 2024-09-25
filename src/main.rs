use clap::Parser;
use anyhow::Result;
mod commands;
mod rpc;
mod utils;

#[derive(Parser)]
#[command(name = "zeek")]
#[command(about = "CLI tool for ZKsync", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    // Handle the command for Proof
    match &cli.command {
        commands::Commands::Proof(proof_args) => {
            commands::proofs::handle_proof(proof_args).await?;
        }
    }

    Ok(())
}