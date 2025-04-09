use crate::types::{Abi, AppConfig, FeeAmount, PoolConfig, RpcConfig, Token};
use std::{env, error::Error};
use tokio::fs;

pub fn get_appconfig(rpc_url: String) -> AppConfig {
    //Initializing beforhand known info
    let usdt = Token {
        address: "0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9"
            .parse()
            .unwrap(),
        symbol: "USDT",
        name: "Tether",
        decimals: 6,
    };

    let usdc = Token {
        address: "0xaf88d065e77c8cC2239327C5EDb3A432268e5831"
            .parse()
            .unwrap(),
        symbol: "USDC",
        name: "USD Coin",
        decimals: 6,
    };

    let pool = PoolConfig {
        address: "0xbE3aD6a5669Dc0B8b12FeBC03608860C31E2eef6"
            .parse()
            .unwrap(),
        token0: usdt,
        token1: usdc,
        fee: FeeAmount::Low,
    };

    AppConfig {
        rpc: RpcConfig { mainnet: (rpc_url) },
        poolcfg: pool,
    }
}

pub async fn initialize_abi(keys: &[&str]) -> Result<Abi, Box<dyn Error>> {
    //Takes the path to ABI from .env and initialize ABIs used for project
    let mut config = Vec::with_capacity(keys.len());

    for &key in keys {
        let path = env::var(key).map_err(|e| format!("Failed to read env key '{key}': {e}"))?;

        let content = fs::read_to_string(&path)
            .await
            .map_err(|e| format!("Failed to read file at {path}: {e}"))?;

        config.push(content);
    }

    Abi::new(&config)
}
