use axum::{Json, extract::{Path}};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, native_token::LAMPORTS_PER_SOL};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct AirdropRequest {
    pub address: String,
}

#[derive(Serialize)]
pub struct AirdropResponse {
    pub address: String,
    pub balance: u64,
}

pub async fn airdrop(Json(payload): Json<AirdropRequest>) -> Json<AirdropResponse> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = Pubkey::from_str(&payload.address).unwrap();

    client.request_airdrop(&pubkey, LAMPORTS_PER_SOL).unwrap();
    let balance = client.get_balance(&pubkey).unwrap();

    Json(AirdropResponse { address: payload.address, balance })
}

pub async fn get_balance(Path(address): Path<String>) -> Json<AirdropResponse> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = Pubkey::from_str(&address).unwrap();
    let balance = client.get_balance(&pubkey).unwrap();

    Json(AirdropResponse { address, balance })
}
