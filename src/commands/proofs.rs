use reqwest::Client;
use clap::Args;

use crate::rpc::proofs as rpc_proofs;
use crate::utils::{proof_verification, visualizer};

#[derive(Args)]
pub struct ProofArgs {
    /// Account address to fetch storage proofs for
    #[arg(short, long)]
    pub address: String,

    /// Storage keys to fetch proofs for
    #[arg(short, long)]
    pub keys: Vec<String>,

    /// L1 batch number
    #[arg(short, long)]
    pub batch: u32,

    /// Verify the proof
    #[arg(short, long)]
    pub verify: bool,

    /// Visualize the Merkle path
    #[arg(short, long)]
    pub visualize: bool,
}

pub async fn handle_proof(args: &ProofArgs) -> anyhow::Result<()> {
    let client = Client::new();

    let proof_result = rpc_proofs::get_proof(&client, &args.address, &args.keys, args.batch).await?;

    println!("Address: {}", proof_result.address);

    for storage_proof in &proof_result.storage_proof {
        println!("Storage Key: {}", storage_proof.key);
        println!("Value: {}", storage_proof.value);
        println!("Index: {}", storage_proof.index);

        if args.verify {
            let batch_details = rpc_proofs::get_l1_batch_details(&client, args.batch).await?;
            let root_hash = batch_details.root_hash;

            let is_valid = proof_verification::verify_proof(storage_proof, &root_hash, &proof_result.address,
                &storage_proof.key,)?;
            println!("Proof Verified: {}", is_valid);
        }

        if args.visualize {
            visualizer::visualize_merkle_path(storage_proof)?;
        }
    }

    Ok(())
}
