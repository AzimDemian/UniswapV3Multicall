mod constants;
mod types;
use alloy_json_abi::JsonAbi;
use alloy_primitives::Address;
use alloy_provider::Provider;
use constants::get_appstate;
use std::env;
use std::fs;

#[tokio::main]

async fn main() {
    dotenvy::dotenv().ok();

    //Load rpc url and provider
    let rpc_url = env::var("RPC_URL")?;
    let provider = Provider::build(&rpc_url)?;

    //Load abi
    let keys = [
        "MULTICALL_ABI_PATH",
        "USDT_ABI_PATH",
        "USDC_ABI_PATH",
        "POOL_ABI_PATH",
    ];

    let abi: Abi = constants::initialize_abi(&keys).unwrap_or_else(|e| {
        panic!("Failed to initialize ABIs: {e}");
    });

    //Initializing config

    let state: types::AppConfig = get_appstate(&rpc_url);

    //Creating pool instance

    let contract = abi.bind(state.poolcfg.address, provider);
}
