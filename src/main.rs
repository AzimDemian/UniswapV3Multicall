mod calls;
mod constants;
mod types;
mod utils;

use alloy::providers::ProviderBuilder;
use calls::multicall::{get_pool_data, make_multicall_contract};
use calls::pool_calls::make_pool_contract;
use constants::{get_appconfig, initialize_abi};
use std::env;
use url::Url;
use utils::log_pool_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    //Load RPC URL and FillProvider from url

    let rpc_url = env::var("RPC_URL")?;
    let url = Url::parse(rpc_url.as_str())?;
    // let client = ClientBuilder::default().http(rpc_url.parse().unwrap());
    // let http_transport = http::Http::with_client(client, url);
    let provider = ProviderBuilder::new().on_http(url);

    //Load ABIs from environment files
    let keys = [
        "MULTICALL_ABI_PATH",
        "USDT_ABI_PATH",
        "USDC_ABI_PATH",
        "POOL_ABI_PATH",
    ];
    let abi = initialize_abi(&keys).await?;

    //Create config and contracts
    let config = get_appconfig(rpc_url);
    let pool_contract = make_pool_contract(config.poolcfg.address, &abi, &provider);
    let multicall_contract = make_multicall_contract(
        "0x5ba1e12693dc8f9c48aad8770482f4739beed696".parse()?,
        &abi,
        &provider,
    );

    //Fetch pool state via multicalls
    let pool_data = get_pool_data(
        &config.poolcfg,
        &config,
        &abi,
        &provider,
        &pool_contract,
        &multicall_contract,
    );

    //Save logs
    log_pool_data(&pool_data)?;

    println!("Pool data written to PoolData.txt");
    Ok(())
}
