use web3::types::{Address, U256};

#[derive(Debug, Clone)]
pub struct Tokend {
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
    pub token0: Tokend,
    pub token1: Tokend,
    pub fee: i32,
}

#[derive(Debug, Clone)]
pub struct PoolData {
    //Struct that will be used to hold final parsed information about Pool
    pub address: Address,
    pub token0: Tokend,
    pub token1: Tokend,
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
