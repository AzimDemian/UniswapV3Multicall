use crate::types::{AppState, FeeAmount, PoolConfig, RpcConfig, Token};
use alloy_primitives::Address;

pub fn get_appstate(rpc_url: &str) -> AppState {
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

    AppState {
        rpc: RpcConfig { mainnet: (rpc_url) },
        poolcfg: pool,
    }
}
