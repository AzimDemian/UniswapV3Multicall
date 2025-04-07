use alloy::json_abi::Abi;
use alloy::json_abi::JsonAbi;
use alloy::primitives::Address;
use alloy::primitives::U256;

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
    pub mainnet: String,
}

#[derive(Debug, Clone)]

pub struct AppConfig {
    pub rpc: RpcConfig,
    pub poolcfg: PoolConfig,
}

#[derive(Debug, Clone)]
pub struct PoolData {
    //Struct for state of Pool that we're intrested in
    pub address: Address,
    pub token0: Token,
    pub token1: Token,
    pub fee: u32,
    pub tick_spacing: i32,
    pub max_liquidity_per_tick: u128,
    pub liquidity: u128,
    pub slot0: Slot0,
    pub ticks: Vec<TickData>,
}

#[derive(Debug, Clone)]
pub struct Slot0 {
    //Struct for the current Slot0 info of the pool
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
    //data of ticks of observed Pool
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
    pub usdt_abi: JsonAbi,
    pub usdc_abi: JsonAbi,
    pub uniswap_pool: JsonAbi,
    pub multicall: JsonAbi,
}

impl Abi {
    pub fn new(config: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        fn parse(name: &str, content: &str) -> alloy::json_abi::Abi {
            serde_json::from_str(content).map_err(|e| format!("Failed to parse {name} ABI: {e}"))?
        }

        Ok(Abi {
            usdt_abi: parse("USDT", &config[0]),
            usdc_abi: parse("USDC", &config[1]),
            uniswap_pool: parse("POOL", &config[2]),
            multicall: parse("MULTICALL", &config[3]),
        })
    }
}
