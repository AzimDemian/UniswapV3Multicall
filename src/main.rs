mod calls;
mod constants;
mod types;
mod utils;

use calls::multicall::get_pool_data;
use constants::get_config;
use std::env;
use utils::{log_pool_data, make_contract};
use web3::transports::Http;
use web3::types::Address;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Load RPC and create provider
    let url = env::var("RPC_URL").map_err(|e| format!("Failed to read env key 'RPC_URL': {e}"))?;
    let http = Http::new(&url).map_err(|e| format!("Failed to build Http: {e}"))?;
    let web3 = web3::Web3::new(http.clone());

    // Load ABI files paths by keys
    let keys = ["POOL_ABI_PATH", "MULTICALL_ABI_PATH"];

    let pool_abi_path =
        env::var(keys[0]).map_err(|e| format!("Failed to read env key 'POOL_ABI_PATH': {e}"))?;

    let multicall_abi_path =
        env::var(keys[1]).map_err(|e| format!("Failed to read env key 'POOL_ABI_PATH': {e}"))?;

    println!("POOL abi: {pool_abi_path}, MULTICALL abi: {multicall_abi_path}");
    // Load pool configuration

    let config = get_config().map_err(|e| format!("Failed to get config: {e}"))?;

    // Create contract instances

    let pool_contract = make_contract(&web3, config.address, &pool_abi_path)
        .await
        .map_err(|e| format!("Failed to create pool contract: {e}"))?;

    let multicall_contract = make_contract(
        &web3,
        "0x5ba1e12693dc8f9c48aad8770482f4739beed696".parse::<Address>()?,
        &multicall_abi_path,
    )
    .await
    .map_err(|e| format!("Failed to create multicall contract: {e}"))?;

    // Fetch pool data
    let pool_data = get_pool_data(&config, &pool_contract, &multicall_contract, &web3)
        .await
        .map_err(|e| format!("Failed to get pool data: {e}"))?;

    // Log pool data to file
    log_pool_data(&pool_data)?;

    println!("Pool data successfully written to PoolData.txt");

    Ok(())
}
