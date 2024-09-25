pub mod proofs;

// Common structs and functions for RPC requests
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RpcRequest<'a> {
    pub jsonrpc: &'static str,
    pub method: &'a str,
    pub params: serde_json::Value,
    pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub result: Option<T>,
    pub error: Option<RpcError>,
    pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}
