use crate::types::Abi;
use alloy::contract::Contract;
use alloy::primitives::{Address, Bytes};
use alloy::providers::{Http, Provider};
use std::error::Error;

pub fn make_pool_contract(addr: Address, abi: &Abi, provider: &Provider<Http>) -> Contract<Http> {
    Contract::new(addr, &abi.uniswap_pool, provider)
}

//prepare fetching slot0 data for multicall
pub fn prepare_slot0_call(contract: &Contract<Http>) -> Result<(Address, Bytes), Box<dyn Error>> {
    let call = contract.method::<_, Bytes>("slot0", ())?;
    Ok((contract.address(), call.calldata()?))
}

//prepare fetching tick spacing data for multicall
pub fn prepare_tick_spacing_call(
    contract: &Contract<Http>,
) -> Result<(Address, Bytes), Box<dyn Error>> {
    let call = contract.method::<_, Bytes>("tickSpacing", ())?;
    Ok((contract.address(), call.calldata()?))
}

//prepare fetching liquidity data for multicall
pub fn prepare_liquidity_call(
    contract: &Contract<Http>,
) -> Result<(Address, Bytes), Box<dyn Error>> {
    let call = contract.method::<_, Bytes>("liquidity", ())?;
    Ok((contract.address(), call.calldata()?))
}

//prepare fetching max liquidity per tick data for multicall
pub fn prepare_max_liquidity_per_tick_call(
    contract: &Contract<Http>,
) -> Result<(Address, Bytes), Box<dyn Error>> {
    let call = contract.method::<_, Bytes>("maxLiquidityPerTick", ())?;
    Ok((contract.address(), call.calldata()?))
}

//prepare fetching bitmap by specific word data for multicall
pub fn prepare_tick_bitmap_call(
    contract: &Contract<Http>,
    word_pos: i16,
) -> Result<(Address, Bytes), Box<dyn Error>> {
    let call = contract.method::<_, Bytes>("tickBitmap", word_pos)?;
    Ok((contract.address(), call.calldata()?))
}

//prepare fetching ticks by index data for multicall
pub fn prepare_tick_call(
    contract: &Contract<Http>,
    tick_index: i32,
) -> Result<(Address, Bytes), Box<dyn Error>> {
    let call = contract.method::<_, Bytes>("ticks", tick_index)?;
    Ok((contract.address(), call.calldata()?))
}
