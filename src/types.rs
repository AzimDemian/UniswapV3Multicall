use crate::utils;
use alloy::json_abi::JsonAbi;
use alloy::primitives::{Address, U256};
#[derive(Debug, Clone)]
pub struct Token {
    //Struct to hold token info (USDT/USDC are basic tokens, they don't take a fee for working with their contracts,
    //so for tokens we know all info beforhand practically)
    pub address: Address,
    pub symbol: &'static str,
    pub name: &'static str,
    pub decimals: u8,
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    //Config of the pool, what we know about Pool beforhand
    pub address: Address,
    pub token0: Token,
    pub token1: Token,
    pub fee: i32,
}

#[derive(Debug, Clone)]
pub struct RpcConfig {
    //To hold the value from .env
    pub mainnet: String,
}

#[derive(Debug, Clone)]

pub struct AppConfig {
    //Config of the app, what we know about app beforhand
    pub rpc: RpcConfig,
    pub poolcfg: PoolConfig,
}

#[derive(Debug, Clone)]
pub struct PoolData {
    //Struct that will be used to hold final parsed information about Pool
    pub address: Address,
    pub token0: Token,
    pub token1: Token,
    pub fee: i32,
    pub tick_spacing: i32,
    pub max_liquidity_per_tick: u128,
    pub liquidity: u128,
    pub slot0: Slot0,
    pub ticks: Vec<TickData>,
}

#[derive(Debug, Clone)]
pub struct Slot0 {
    //Struct that will be used to destruct Slot0 info
    pub sqrt_price_x96: U256,
    pub tick: i32,
    pub observation_index: u16,
    pub observation_cardinality: u16,
    pub observation_cardinality_next: u16,
    pub fee_protocol: u8,
    pub unlocked: bool,
}

#[derive(Debug, Clone)]
pub struct TickData {
    //Struct that will be used to destruct data about single Tick
    pub tick_index: i32,
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
    pub fee_growth_outside_0_x128: U256,
    pub fee_growth_outside_1_x128: U256,
    pub tick_cumulative_outside: i64,
    pub seconds_per_liquidity_outside_x128: u128,
    pub seconds_outside: u32,
    pub initialized: bool,
}

#[derive(Debug, Clone)]

pub struct Abi {
    ///Struct that will be used to store Abi used in project
    pub usdt_abi: JsonAbi,
    pub usdc_abi: JsonAbi,
    pub uniswap_pool: JsonAbi,
    pub multicall: JsonAbi,
}

impl Abi {
    pub fn new(config: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Abi {
            usdt_abi: utils::parse("USDT", &config[0])?,
            usdc_abi: utils::parse("USDC", &config[1])?,
            uniswap_pool: utils::parse("POOL", &config[2])?,
            multicall: utils::parse("MULTICALL", &config[3])?,
        })
    }
}
