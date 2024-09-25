use serde::{Deserialize};

use crate::rpc::{RpcRequest, RpcResponse};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorageProof {
    pub key: String,
    pub value: String,
    pub index: u64,
    pub proof: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProofResult {
    pub address: String,
    pub storage_proof: Vec<StorageProof>,
}

pub async fn get_proof(
    client: &reqwest::Client,
    address: &str,
    keys: &Vec<String>,
    batch: u32,
) -> anyhow::Result<ProofResult> {
    let rpc_request = RpcRequest {
        jsonrpc: "2.0",
        method: "zks_getProof",
        params: serde_json::json!([address, keys, batch]),
        id: 1,
    };

    let response = client
        .post("https://mainnet.era.zksync.io")
        .json(&rpc_request)
        .send()
        .await?;

    let rpc_response: RpcResponse<ProofResult> = response.json().await?;

    if let Some(error) = rpc_response.error {
        return Err(anyhow::anyhow!("RPC Error {}: {}", error.code, error.message));
    }

    Ok(rpc_response.result.unwrap())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1BatchDetails {
    pub number: u32,
    pub timestamp: u64,
    pub l1_tx_count: u32,
    pub l2_tx_count: u32,
    pub root_hash: String,
    pub status: String,
    pub commit_tx_hash: Option<String>,
    pub committed_at: Option<String>,
    pub prove_tx_hash: Option<String>,
    pub proven_at: Option<String>,
    pub execute_tx_hash: Option<String>,
    pub executed_at: Option<String>,
    pub l1_gas_price: u64,
    pub l2_fair_gas_price: u64,
    pub base_system_contracts_hashes: BaseSystemContractsHashes,
}
#[derive(Deserialize, Debug)]
pub struct BaseSystemContractsHashes {
    pub bootloader: String,
    pub default_aa: String,
}

pub async fn get_l1_batch_details(
    client: &reqwest::Client,
    batch_number: u32,
) -> anyhow::Result<L1BatchDetails> {
    let rpc_request = RpcRequest {
        jsonrpc: "2.0",
        method: "zks_getL1BatchDetails",
        params: serde_json::json!([batch_number]),
        id: 1,
    };

    let response = client
        .post("https://mainnet.era.zksync.io")
        .json(&rpc_request)
        .send()
        .await?;

    let rpc_response: RpcResponse<L1BatchDetails> = response.json().await?;

    if let Some(error) = rpc_response.error {
        return Err(anyhow::anyhow!(
            "RPC Error {}: {}",
            error.code,
            error.message
        ));
    }

    Ok(rpc_response.result.unwrap())
}

