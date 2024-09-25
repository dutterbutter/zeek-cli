pub mod proofs;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Generate, verify, and visualize Merkle proofs
    Proof(proofs::ProofArgs),
}
