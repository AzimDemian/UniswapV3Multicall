mod constants;
mod types;
use alloy::json_abi::JsonAbi;
use alloy::primitives::Address;
use alloy::providers::{Http, Provider};
use std::str::FromStr;

use constants::get_appstate;
use std::env;
use std::fs;

#[tokio::main]

async fn main() {
    dotenv::dotenv().ok();

    //Load rpc url and provider
    let rpc_url = env::var("RPC_URL")?;
    let provider = Provider::<Http>::try_from(rpc_url)?;

    //Load abi
    let keys = [
        "MULTICALL_ABI_PATH",
        "USDT_ABI_PATH",
        "USDC_ABI_PATH",
        "POOL_ABI_PATH",
    ];

    let abi = constants::initialize_abi(&keys).await.unwrap_or_else(|e| {
        eprintln!("ABI initialization failed: {e}");
        std::process::exit(1);
    });

    //Initializing config

    let state: types::AppConfig = get_appstate(&rpc_url);

    //Creating pool instance

    let contract = abi.bind(state.poolcfg.address, provider);
}
