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

    //Load abi of pool
    let abi_str = file::read_to_string("abi/uniswap_pool.json")?;
    let abi: JsonAbi = serde_json::from_str(&abi_str)?;

    //Initializing config

    let state: types::AppConfig = get_appstate(&rpc_url);

    //Creating pool instance

    let contract = abi.bind(state.poolcfg.address, provider);
}
