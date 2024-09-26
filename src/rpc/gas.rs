use crate::rpc::{RpcRequest, RpcResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CallRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Deserialize)]
pub struct FeeEstimate {
    pub gas_limit: String,
    pub max_fee_per_gas: String,
    pub max_priority_fee_per_gas: String,
    pub gas_per_pubdata_limit: String,
}

impl FeeEstimate {
    pub fn parse_gas_limit(&self) -> anyhow::Result<u64> {
        Ok(u64::from_str_radix(
            self.gas_limit.trim_start_matches("0x"),
            16,
        )?)
    }

    pub fn parse_max_fee_per_gas(&self) -> anyhow::Result<u64> {
        Ok(u64::from_str_radix(
            self.max_fee_per_gas.trim_start_matches("0x"),
            16,
        )?)
    }

    pub fn parse_max_priority_fee_per_gas(&self) -> anyhow::Result<u64> {
        Ok(u64::from_str_radix(
            self.max_priority_fee_per_gas.trim_start_matches("0x"),
            16,
        )?)
    }

    pub fn parse_gas_per_pubdata_limit(&self) -> anyhow::Result<u64> {
        Ok(u64::from_str_radix(
            self.gas_per_pubdata_limit.trim_start_matches("0x"),
            16,
        )?)
    }
}

#[derive(Deserialize)]
pub struct FeeParamsResponse {
    pub V2: Option<FeeParamsV2>,
}

#[derive(Deserialize)]
pub struct FeeParamsResult {
    pub V2: Option<FeeParamsV2>,
}

#[derive(Deserialize)]
pub struct FeeParamsV2 {
    pub config: FeeConfig,
    pub l1_gas_price: u64,
    pub l1_pubdata_price: u64,
}

#[derive(Deserialize)]
pub struct FeeConfig {
    pub minimal_l2_gas_price: u64,
    pub compute_overhead_part: f64,
    pub pubdata_overhead_part: f64,
    pub batch_overhead_l1_gas: u64,
    pub max_gas_per_batch: u64,
    pub max_pubdata_per_batch: u64,
}

pub async fn estimate_fee(
    client: &reqwest::Client,
    call_request: &CallRequest,
    rpc_url: &str,
) -> anyhow::Result<FeeEstimate> {
    let rpc_request = RpcRequest {
        jsonrpc: "2.0",
        method: "zks_estimateFee",
        params: vec![serde_json::to_value(call_request)?],
        id: 1,
    };

    let response = client.post(rpc_url).json(&rpc_request).send().await?;

    let rpc_response: RpcResponse<FeeEstimate> = response.json().await?;

    if let Some(error) = rpc_response.error {
        return Err(anyhow::anyhow!(
            "RPC Error {}: {}",
            error.code,
            error.message
        ));
    }

    rpc_response
        .result
        .ok_or_else(|| anyhow::anyhow!("No result in RPC response"))
}

pub async fn calculate_pubdata_cost(
    client: &reqwest::Client,
    fee_estimate: &FeeEstimate,
    rpc_url: &str,
) -> anyhow::Result<f64> {
    // Retrieve fee parameters using zks_getFeeParams
    let rpc_request = RpcRequest {
        jsonrpc: "2.0",
        method: "zks_getFeeParams",
        params: vec![],
        id: 1,
    };

    let response = client.post(rpc_url).json(&rpc_request).send().await?;

    let rpc_response: RpcResponse<FeeParamsResponse> = response.json().await?;

    let fee_params = rpc_response
        .result
        .ok_or_else(|| anyhow::anyhow!("No result in RPC response"))?
        .V2
        .ok_or_else(|| anyhow::anyhow!("Missing V2 in fee parameters"))?;

    let l1_pubdata_price = fee_params.l1_pubdata_price;

    // Parse gas_per_pubdata_limit
    let gas_per_pubdata_limit = fee_estimate.parse_gas_per_pubdata_limit()?;

    // Calculate pubdata cost: (gas_per_pubdata_limit * l1_pubdata_price)
    let pubdata_cost_wei = gas_per_pubdata_limit as u128 * l1_pubdata_price as u128;

    // Convert Wei to ETH
    let pubdata_cost_eth = pubdata_cost_wei as f64 / 1e18;

    Ok(pubdata_cost_eth)
}

pub async fn estimate_gas_l1_to_l2(
    client: &reqwest::Client,
    call_request: &CallRequest,
    rpc_url: &str,
) -> anyhow::Result<u64> {
    let rpc_request = RpcRequest {
        jsonrpc: "2.0",
        method: "zks_estimateGasL1ToL2",
        params: vec![serde_json::to_value(call_request)?],
        id: 1,
    };

    let response = client.post(rpc_url).json(&rpc_request).send().await?;

    let rpc_response: RpcResponse<String> = response.json().await?;

    if let Some(error) = rpc_response.error {
        return Err(anyhow::anyhow!(
            "RPC Error {}: {}",
            error.code,
            error.message
        ));
    }

    let gas_hex = rpc_response
        .result
        .ok_or_else(|| anyhow::anyhow!("No result in RPC response"))?;

    let gas_estimate = u64::from_str_radix(gas_hex.trim_start_matches("0x"), 16)?;

    Ok(gas_estimate)
}

pub async fn get_fee_params(
    client: &reqwest::Client,
    rpc_url: &str,
) -> anyhow::Result<FeeParamsV2> {
    let rpc_request = RpcRequest {
        jsonrpc: "2.0",
        method: "zks_getFeeParams",
        params: vec![],
        id: 1,
    };

    let response = client.post(rpc_url).json(&rpc_request).send().await?;

    let rpc_response: RpcResponse<FeeParamsResponse> = response.json().await?;

    if let Some(error) = rpc_response.error {
        return Err(anyhow::anyhow!(
            "RPC Error {}: {}",
            error.code,
            error.message
        ));
    }

    let fee_params = rpc_response
        .result
        .ok_or_else(|| anyhow::anyhow!("No result in RPC response"))?
        .V2
        .ok_or_else(|| anyhow::anyhow!("Missing V2 in fee parameters"))?;

    Ok(fee_params)
}

pub async fn get_l1_gas_price(client: &reqwest::Client, rpc_url: &str) -> anyhow::Result<u64> {
    let rpc_request = RpcRequest {
        jsonrpc: "2.0",
        method: "zks_getL1GasPrice",
        params: vec![],
        id: 1,
    };

    let response = client.post(rpc_url).json(&rpc_request).send().await?;

    let rpc_response: RpcResponse<String> = response.json().await?;

    if let Some(error) = rpc_response.error {
        return Err(anyhow::anyhow!(
            "RPC Error {}: {}",
            error.code,
            error.message
        ));
    }

    let gas_price_hex = rpc_response
        .result
        .ok_or_else(|| anyhow::anyhow!("No result in RPC response"))?;

    let gas_price = u64::from_str_radix(gas_price_hex.trim_start_matches("0x"), 16)?;

    Ok(gas_price)
}

pub async fn get_gas_price(client: &reqwest::Client, rpc_url: &str) -> anyhow::Result<u64> {
    let rpc_request = RpcRequest {
        jsonrpc: "2.0",
        method: "eth_gasPrice",
        params: vec![],
        id: 1,
    };

    let response = client.post(rpc_url).json(&rpc_request).send().await?;

    let rpc_response: RpcResponse<String> = response.json().await?;

    if let Some(error) = rpc_response.error {
        return Err(anyhow::anyhow!(
            "RPC Error {}: {}",
            error.code,
            error.message
        ));
    }

    let gas_price_hex = rpc_response
        .result
        .ok_or_else(|| anyhow::anyhow!("No result in RPC response"))?;

    let gas_price = u64::from_str_radix(gas_price_hex.trim_start_matches("0x"), 16)?;

    Ok(gas_price)
}
