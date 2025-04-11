use crate::types::{PoolData, Slot0, TickData};
use chrono::Local;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::Write;
use web3::contract::Contract;
use web3::ethabi::{Function, Token};
use web3::transports::Http;
use web3::types::{Address, Bytes, U256};

pub fn log_pool_data(pool: &PoolData) -> std::io::Result<()> {
    //Function used to log current state of Pool into file PoolData.txt
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("PoolData.txt")?;

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    writeln!(file, "Time: {now}")?;
    writeln!(file, "Pool Address: {:?}", pool.address)?;
    writeln!(file, "Fee: {} (in hundredths of a bip)", pool.fee)?;
    writeln!(file, "Tick Spacing: {}", pool.tick_spacing)?;
    writeln!(
        file,
        "Max Liquidity per Tick: {}",
        pool.max_liquidity_per_tick
    )?;
    writeln!(file, "Current Liquidity: {}", pool.liquidity)?;

    writeln!(file, "\nToken 0:")?;
    writeln!(file, "  Address: {:?}", pool.token0.address)?;
    writeln!(file, "  Symbol: {}", pool.token0.symbol)?;
    writeln!(file, "  Name: {}", pool.token0.name)?;
    writeln!(file, "  Decimals: {}", pool.token0.decimals)?;

    writeln!(file, "\nToken 1:")?;
    writeln!(file, "  Address: {:?}", pool.token1.address)?;
    writeln!(file, "  Symbol: {}", pool.token1.symbol)?;
    writeln!(file, "  Name: {}", pool.token1.name)?;
    writeln!(file, "  Decimals: {}", pool.token1.decimals)?;

    writeln!(file, "\nSlot0 Info:")?;
    writeln!(file, "  Sqrt Price X96: {}", pool.slot0.sqrt_price_x96)?;
    writeln!(file, "  Tick: {}", pool.slot0.tick)?;
    writeln!(
        file,
        "  Observation Index: {}",
        pool.slot0.observation_index
    )?;
    writeln!(
        file,
        "  Observation Cardinality: {}",
        pool.slot0.observation_cardinality
    )?;
    writeln!(
        file,
        "  Observation Cardinality Next: {}",
        pool.slot0.observation_cardinality_next
    )?;
    writeln!(file, "  Fee Protocol: {}", pool.slot0.fee_protocol)?;
    writeln!(file, "  Unlocked: {}", pool.slot0.unlocked)?;

    writeln!(file, "\nInitialized Ticks: {}", pool.ticks.len())?;
    for tick in &pool.ticks {
        writeln!(file, "  Tick Index: {}", tick.tick_index)?;
        writeln!(file, "    Liquidity Gross: {}", tick.liquidity_gross)?;
        writeln!(file, "    Liquidity Net: {}", tick.liquidity_net)?;
        writeln!(
            file,
            "    Fee Growth Outside 0: {}",
            tick.fee_growth_outside_0_x128
        )?;
        writeln!(
            file,
            "    Fee Growth Outside 1: {}",
            tick.fee_growth_outside_1_x128
        )?;
        writeln!(
            file,
            "    Tick Cumulative Outside: {}",
            tick.tick_cumulative_outside
        )?;
        writeln!(
            file,
            "    Seconds Per Liquidity Outside: {}",
            tick.seconds_per_liquidity_outside_x128
        )?;
        writeln!(file, "    Seconds Outside: {}", tick.seconds_outside)?;
        writeln!(file, "    Initialized: {}", tick.initialized)?;
    }

    writeln!(file, "\n---\n")?;
    Ok(())
}

pub fn calculate_bitmap_word_positions(
    //Function used to calculate all possible bitmap positions
    min_tick: i32,
    max_tick: i32,
    tick_spacing: i32,
) -> Result<Vec<i16>, Box<dyn Error>> {
    let mut positions = HashSet::new();
    let mut tick = min_tick;

    while tick <= max_tick {
        let normalized_tick = tick / tick_spacing;
        let word_pos = normalized_tick / 256;

        if word_pos < i16::MIN as i32 || word_pos > i16::MAX as i32 {
            return Err(format!("Bitmap word_pos {} out of i16 bounds", word_pos).into());
        }

        positions.insert(word_pos as i16);
        tick += tick_spacing;
    }

    let mut result: Vec<i16> = positions.into_iter().collect();
    result.sort_unstable();
    Ok(result)
}

pub fn get_initialized_ticks(
    word_positions: &[i16],
    bitmaps: &[U256],
    tick_spacing: i32,
) -> Vec<i32> {
    let mut initialized_ticks = Vec::new();

    for (j, &bitmap) in bitmaps.iter().enumerate() {
        if bitmap == U256::zero() {
            continue;
        }

        let word_index = word_positions[j] as i32;

        for i in 0..256 {
            let bit = U256::from(1) << i;
            let is_initialized = bitmap & bit != U256::zero();

            if is_initialized {
                let tick_index = (word_index * 256 + i) * tick_spacing;
                initialized_ticks.push(tick_index);
            }
        }
    }

    initialized_ticks
}

pub async fn make_contract(
    web3: &web3::Web3<Http>,
    addr: Address,
    abi_path: &str,
) -> Result<Contract<Http>, Box<dyn Error>> {
    let abi_json = fs::read_to_string(abi_path)?;
    let contract = Contract::from_json(web3.eth(), addr, abi_json.as_bytes())?;
    Ok(contract)
}

pub fn decode_call_result(
    contract: &Contract<Http>,
    method_name: &str,
    data: &Bytes,
) -> Result<Vec<Token>, Box<dyn Error>> {
    let func: &Function = contract
        .abi()
        .function(method_name)
        .map_err(|e| format!("Function {} not found: {}", method_name, e))?;

    let decoded = func
        .decode_output(&data.0)
        .map_err(|e| format!("Decode failed for {}: {}", method_name, e))?;

    Ok(decoded)
}

pub fn decode_u128_token(
    contract: &Contract<Http>,
    method: &str,
    data: &Bytes,
) -> Result<u128, Box<dyn Error>> {
    let tokens = decode_call_result(contract, method, data)?;
    match tokens.get(0) {
        Some(Token::Uint(val)) => Ok(val.as_u128()),
        _ => Err("Expected u128 token".into()),
    }
}

pub fn decode_i32_token(
    contract: &Contract<Http>,
    method: &str,
    data: &Bytes,
) -> Result<i32, Box<dyn Error>> {
    let tokens = decode_call_result(contract, method, data)?;
    match tokens.get(0) {
        Some(Token::Int(val)) => Ok(val.low_u32() as i32),
        _ => Err("Expected i32 token".into()),
    }
}

pub fn decode_slot0_tokens(
    contract: &Contract<Http>,
    method: &str,
    data: &Bytes,
) -> Result<Slot0, Box<dyn Error>> {
    let tokens = decode_call_result(contract, method, data)?;
    if let [
        Token::Uint(sqrt_price_x96),
        Token::Int(tick),
        Token::Uint(observation_index),
        Token::Uint(observation_cardinality),
        Token::Uint(observation_cardinality_next),
        Token::Uint(fee_protocol),
        Token::Bool(unlocked),
    ] = tokens.as_slice()
    {
        Ok(Slot0 {
            sqrt_price_x96: (*sqrt_price_x96).into(),
            tick: tick.low_u32() as i32,
            observation_index: observation_index.low_u32() as u16,
            observation_cardinality: observation_cardinality.low_u32() as u16,
            observation_cardinality_next: observation_cardinality_next.low_u32() as u16,
            fee_protocol: fee_protocol.low_u32() as u8,
            unlocked: *unlocked,
        })
    } else {
        Err("Unexpected Slot0 token structure".into())
    }
}

pub fn decode_u256_token(
    contract: &Contract<Http>,
    method: &str,
    data: &Bytes,
) -> Result<U256, Box<dyn Error>> {
    let tokens = decode_call_result(contract, method, data)?;
    match tokens.get(0) {
        Some(Token::Uint(val)) => Ok(*val),
        _ => Err(format!("Expected Token::Uint from {}", method).into()),
    }
}

pub fn decode_ticks(
    indices: &[i32],
    raw_results: &[Bytes],
    pool: &Contract<Http>,
) -> Result<Vec<TickData>, Box<dyn Error>> {
    let mut decoded = Vec::with_capacity(raw_results.len());

    for (i, bytes) in raw_results.iter().enumerate() {
        let tokens = decode_call_result(pool, "ticks", bytes)?;

        if let [
            Token::Uint(liquidity_gross),
            Token::Int(liquidity_net),
            Token::Uint(fee_growth_outside_0),
            Token::Uint(fee_growth_outside_1),
            Token::Int(tick_cumulative_outside),
            Token::Uint(seconds_per_liquidity_outside),
            Token::Uint(seconds_outside),
            Token::Bool(initialized),
        ] = tokens.as_slice()
        {
            decoded.push(TickData {
                tick_index: indices[i],
                liquidity_gross: liquidity_gross.as_u128(),
                liquidity_net: liquidity_net.low_u128() as i128,
                fee_growth_outside_0_x128: *fee_growth_outside_0,
                fee_growth_outside_1_x128: *fee_growth_outside_1,
                tick_cumulative_outside: tick_cumulative_outside.low_u64() as i64,
                seconds_per_liquidity_outside_x128: seconds_per_liquidity_outside.as_u128(),
                seconds_outside: seconds_outside.as_u32(),
                initialized: *initialized,
            });
        } else {
            return Err(format!("Unexpected ticks token structure at index {}", i).into());
        }
    }

    Ok(decoded)
}
