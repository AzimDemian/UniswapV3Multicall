//Builder  of multicalls

use alloy::{
    contract::{ContractInstance, Interface}, 
    primitives::{Address, Bytes, I256}, 
    providers::fill::FillProvider,
    dyn_abi::DynSolValue,
};
use std::error::Error;
use super::pool_calls::{self, prepare_call};
use crate::types::{Abi, PoolConfig, AppConfig, PoolData};



pub fn make_multicall_contract(addr: Address, abi: &Abi, provider: &FillProvider) -> ContractInstance<FillProvider>{ 
    //Initializing multicall contract
    let interface = Interface::new(abi.multicall.clone());
    ContractInstance::new(addr, provider.clone(), interface)
}

pub async fn initial_multicall(multicall_contract: &ContractInstance<FillProvider>, pool_contract: &ContractInstance<FillProvider>) -> Result<Vec<DynSolValue> , Box<dyn Error> >{
    //Calling everything that can be straightforward put in PoolData and what will be used to calculate initialized Ticks
    let calls: Vec<(Address, Bytes)> = vec![
        pool_calls::prepare_call(pool_contract, "slot0", &[]).unwrap(),
        pool_calls::prepare_call(pool_contract, "tickSpacing", &[]).unwrap(),
        pool_calls::prepare_call(pool_contract, "liquidity", &[]).unwrap(),
        pool_calls::prepare_call(pool_contract, "maxLiquidityPerTick", &[]).unwrap(),   
    ];

    //Collecting arguments from prepared calls for single multicall
    let multicall_args: Vec<DynSolValue> = calls.iter()
        .map(|(address, calldata)| {
            DynSolValue::Tuple(vec![
                DynSolValue::Address(*address),
                DynSolValue::Bytes(calldata.to_vec()),
            ])
        })
        .collect();

    //Proccessing with multicall
    let result = multicall_contract
        .function("aggregate", &[DynSolValue::Array(multicall_args)])?
        .call()
        .await?; 

    Ok(result)
}

pub async fn fetch_all_bitmaps(
    words: Vec<i16>, //We'll calculate with utils::calculate_bitmap_word_positions 
    multicall_contract: &ContractInstance<FillProvider>,
    pool_contract: &ContractInstance<FillProvider>,
) -> Result<Vec<DynSolValue>, Box<dyn Error>> {
    let calls: Vec<(Address, Bytes)> = words
        .iter()
        .map(|i| {
            prepare_call(
                pool_contract,
                "tickBitmap",
                &[DynSolValue::Int(I256::from(*i), 256)],
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    let multicall_args: Vec<DynSolValue> = calls
        .iter()
        .map(|(address, calldata)| {
            DynSolValue::Tuple(vec![
                DynSolValue::Address(*address),
                DynSolValue::Bytes(calldata.to_vec()),
            ])
        })
        .collect();

    let result = multicall_contract
        .function("aggregate", &[DynSolValue::Array(multicall_args)])?
        .call()
        .await?;

    Ok(result)
}

pub async fn fetch_all_ticks(
    tick_indices: &[i32],
    multicall_contract: &ContractInstance<FillProvider>,
    pool_contract: &ContractInstance<FillProvider>,
) -> Result<Vec<DynSolValue>, Box<dyn Error>> {
    let calls: Vec<(Address, Bytes)> = tick_indices
        .iter()
        .map(|i| {
            prepare_call(
                pool_contract,
                "ticks",
                &[DynSolValue::Int(I256::from(*i), 24)],
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    let multicall_args: Vec<DynSolValue> = calls
        .iter()
        .map(|(address, calldata)| {
            DynSolValue::Tuple(vec![
                DynSolValue::Address(*address),
                DynSolValue::Bytes(calldata.to_vec()),
            ])
        })
        .collect();

    let result = multicall_contract
        .function("aggregate", &[DynSolValue::Array(multicall_args)])?
        .call()
        .await?;

    Ok(result)
}

pub fn get_pool_data(poolcfg: PoolConfig, app_state: AppConfig) -> PoolData {

    let usdt_data = 

    PoolData {
        address: poolcfg.address,
        token0: app_state.pool.token0,
        token1: app_state.pool.token1,
        fee: poolcfg.fee,
        tick_spacing: ,
        max_liquidity_per_tick: ,
        liquidity: ,
        slot0: ,
        protocol_fees: ,
        ticks: 

    }
} //function that returns data that we're intreseted in
