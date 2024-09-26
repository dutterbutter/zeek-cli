use crate::rpc::gas as rpc_gas;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum GasCommands {
    /// Estimate the fee for a transaction
    EstimateFee(EstimateFeeArgs),
    /// Estimate gas for L1 to L2 transactions
    EstimateGasL1ToL2(EstimateGasL1ToL2Args),
    /// Get current fee parameters
    FeeParams,
    /// Get current L1 gas price
    L1GasPrice,
    /// Get current L2 gas price
    GasPrice,
}

#[derive(Args)]
pub struct EstimateFeeArgs {
    /// Transaction recipient address
    #[arg(short, long)]
    pub to: Option<String>,

    /// Transaction data
    #[arg(short, long, default_value = "0x")]
    pub data: String,

    /// Sender address
    #[arg(short, long)]
    pub from: Option<String>,

    /// Value to send (in ETH)
    #[arg(short, long)]
    pub value: Option<f64>,

    /// Gas limit
    #[arg(long)]
    pub gas_limit: Option<u64>,

    /// Gas price (in Gwei)
    #[arg(long)]
    pub gas_price: Option<f64>,

    /// Show pubdata costs
    #[arg(long)]
    pub show_pubdata: bool,
}

#[derive(Args)]
pub struct EstimateGasL1ToL2Args {
    /// Transaction recipient address
    #[arg(short, long)]
    pub to: String,

    /// Transaction data
    #[arg(short, long, default_value = "0x")]
    pub data: String,

    /// Sender address
    #[arg(short, long)]
    pub from: Option<String>,

    /// Value to send (in ETH)
    #[arg(short, long)]
    pub value: Option<f64>,
}

pub async fn handle_gas_command(cmd: &GasCommands, rpc_url: &str) -> anyhow::Result<()> {
    match cmd {
        GasCommands::EstimateFee(args) => {
            handle_estimate_fee(args, rpc_url).await?;
        }
        GasCommands::EstimateGasL1ToL2(args) => {
            handle_estimate_gas_l1_to_l2(args, rpc_url).await?;
        }
        GasCommands::FeeParams => {
            handle_get_fee_params(rpc_url).await?;
        }
        GasCommands::L1GasPrice => {
            handle_get_l1_gas_price(rpc_url).await?;
        }
        GasCommands::GasPrice => {
            handle_get_gas_price(rpc_url).await?;
        }
    }
    Ok(())
}

async fn handle_estimate_fee(args: &EstimateFeeArgs, rpc_url: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    // Prepare the call request parameters
    let call_request = rpc_gas::CallRequest {
        from: args.from.clone(),
        to: args.to.clone(),
        gas: args.gas_limit.map(|g| format!("0x{:x}", g)),
        gas_price: args
            .gas_price
            .map(|gp| format!("0x{:x}", (gp * 1e9) as u64)), // Gwei to Wei
        value: args.value.map(|v| format!("0x{:x}", (v * 1e18) as u64)), // ETH to Wei
        data: Some(args.data.clone()),
    };

    // Estimate the fee
    let fee_estimate = rpc_gas::estimate_fee(&client, &call_request, rpc_url).await?;
    let gas_limit = fee_estimate.parse_gas_limit()?;
    let max_fee_per_gas = fee_estimate.parse_max_fee_per_gas()?;
    let max_priority_fee_per_gas = fee_estimate.parse_max_priority_fee_per_gas()?;
    let gas_per_pubdata_limit = fee_estimate.parse_gas_per_pubdata_limit()?;

    // Display the results with proper formatting
    println!("Gas Limit: {}", gas_limit);
    println!("Max Fee Per Gas: {:.2} Gwei", max_fee_per_gas as f64 / 1e9);
    println!(
        "Max Priority Fee Per Gas: {:.2} Gwei",
        max_priority_fee_per_gas as f64 / 1e9
    );
    println!("Gas Per Pubdata Limit: {}", gas_per_pubdata_limit);

    if args.show_pubdata {
        let pubdata_cost = rpc_gas::calculate_pubdata_cost(&client, &fee_estimate, rpc_url).await?;
        println!("Pubdata Cost: {:.8} ETH", pubdata_cost);
    }

    Ok(())
}

async fn handle_estimate_gas_l1_to_l2(
    args: &EstimateGasL1ToL2Args,
    rpc_url: &str,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let call_request = rpc_gas::CallRequest {
        from: args.from.clone(),
        to: Some(args.to.clone()),
        gas: None,
        gas_price: None,
        value: args.value.map(|v| format!("0x{:x}", (v * 1e18) as u64)), // ETH to Wei
        data: Some(args.data.clone()),
    };

    let gas_estimate = rpc_gas::estimate_gas_l1_to_l2(&client, &call_request, rpc_url).await?;

    println!("Estimated Gas for L1 to L2 Transaction: {}", gas_estimate);

    Ok(())
}

async fn handle_get_fee_params(rpc_url: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let fee_params = rpc_gas::get_fee_params(&client, rpc_url).await?;

    println!("Fee Parameters:");
    println!(
        "Minimal L2 Gas Price: {}",
        fee_params.config.minimal_l2_gas_price
    );
    println!(
        "Compute Overhead Part: {}",
        fee_params.config.compute_overhead_part
    );
    println!(
        "Pubdata Overhead Part: {}",
        fee_params.config.pubdata_overhead_part
    );
    println!(
        "Batch Overhead L1 Gas: {}",
        fee_params.config.batch_overhead_l1_gas
    );
    println!("Max Gas Per Batch: {}", fee_params.config.max_gas_per_batch);
    println!(
        "Max Pubdata Per Batch: {}",
        fee_params.config.max_pubdata_per_batch
    );
    println!("L1 Gas Price: {}", fee_params.l1_gas_price);
    println!("L1 Pubdata Price: {}", fee_params.l1_pubdata_price);

    Ok(())
}

async fn handle_get_l1_gas_price(rpc_url: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let gas_price = rpc_gas::get_l1_gas_price(&client, rpc_url).await?;

    println!("Current L1 Gas Price: {:.2} Gwei", gas_price as f64 / 1e9);

    Ok(())
}

async fn handle_get_gas_price(rpc_url: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let gas_price = rpc_gas::get_gas_price(&client, rpc_url).await?;

    println!("Current L2 Gas Price: {:.2} Gwei", gas_price as f64 / 1e9);

    Ok(())
}
