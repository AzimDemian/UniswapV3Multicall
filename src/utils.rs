use crate::types::PoolData;
use alloy::dyn_abi::SolType;
use alloy::primitives::{Bytes, U256};
use chrono::Local;
use std::collections::HashSet;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_pool_data(pool: &PoolData) -> std::io::Result<()> {
    //Function used to log current state of Pool into file PoolData.txt
    let mut file = OpenOptions::new()
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
        if bitmap == U256::ZERO {
            continue;
        }

        let word_index = word_positions[j] as i32;

        for i in 0..256 {
            let bit = U256::from(1) << i;
            let is_initialized = bitmap & bit != U256::ZERO;

            if is_initialized {
                let tick_index = (word_index * 256 + i) * tick_spacing;
                initialized_ticks.push(tick_index);
            }
        }
    }

    initialized_ticks
}

pub fn decode_response<T: SolType>(
    data: Bytes,
) -> Result<<T as SolType>::RustType, Box<dyn std::error::Error>> {
    T::abi_decode(&data, true).map_err(|e| e.into())
}

pub fn parse(
    //Parser of JSON Abi
    name: &str,
    content: &str,
) -> Result<alloy::json_abi::JsonAbi, Box<dyn std::error::Error>> {
    serde_json::from_str(content).map_err(|e| format!("Failed to parse {name} ABI: {e}").into())
}
