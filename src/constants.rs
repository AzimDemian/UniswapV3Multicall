use crate::types::{PoolConfig, Tokend};
use std::error::Error;

pub fn get_config() -> Result<PoolConfig, Box<dyn Error>> {
    //Initializing beforhand known info
    let usdt = Tokend {
        address: "0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9"
            .parse()
            .unwrap(),
        symbol: "USDT",
        name: "Tether",
        decimals: 6,
    };

    let usdc = Tokend {
        address: "0xaf88d065e77c8cC2239327C5EDb3A432268e5831"
            .parse()
            .unwrap(),
        symbol: "USDC",
        name: "USD Coin",
        decimals: 6,
    };

    let pool = PoolConfig {
        address: "0x7858E59e0C01EA06Df3aF3D20aC7B0003275D4Bf"
            .parse()
            .unwrap(),
        token0: usdt,
        token1: usdc,
        fee: 500,
    };

    Ok(pool)
}
