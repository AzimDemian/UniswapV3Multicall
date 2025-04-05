use alloy_primitives::Address;

#[derive(Debug, Clone)]
pub struct Token {
    //Token info
    pub address: Address,
    pub symbol: &'static str,
    pub name: &'static str,
    pub decimals: u8,
}

#[derive(Debug, Clone)]

pub enum FeeAmount {
    Low = 100,      // 0.01%
    Medium0 = 500,  //0.05%
    Medium1 = 3000, // 0.3%
    High = 10000,   // 1%
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub address: Address,
    pub token0: Token,
    pub token1: Token,
    pub fee: FeeAmount,
}

#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub mainnet: &'static str,
}

#[derive(Debug, Clone)]

pub struct AppConfig {
    pub rpc: RpcConfig,
    pub poolcfg: PoolConfig,
}
