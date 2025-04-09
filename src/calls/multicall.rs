//Builder  of multicalls

use super::pool_calls::{self, prepare_call};
use crate::{
    types::{Abi, AppConfig, PoolConfig, PoolData},
    utils,
};
use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    network::Ethereum,
    primitives::{Address, Bytes},
    providers::Provider,
};
use std::error::Error;

pub fn make_multicall_contract<P>(addr: Address, abi: &Abi, provider: &P) -> ContractInstance<P>
where
    P: Provider<Ethereum> + Clone,
{
    //Initializing multicall contract
    let interface = Interface::new(abi.multicall.clone());
    ContractInstance::new(addr, provider.clone(), interface)
}

pub async fn initial_multicall<P>(
    multicall_contract: &ContractInstance<P>,
    pool_contract: &ContractInstance<P>,
) -> Result<Vec<DynSolValue>, Box<dyn Error>>
where
    P: Provider<Ethereum> + Clone,
{
    //Calling everything that can be straightforward put in PoolData and what will be used to calculate initialized Ticks
    let calls: Vec<(Address, Bytes)> = vec![
        pool_calls::prepare_call(pool_contract, "slot0", &[]).unwrap(),
        pool_calls::prepare_call(pool_contract, "tickSpacing", &[]).unwrap(),
        pool_calls::prepare_call(pool_contract, "liquidity", &[]).unwrap(),
        pool_calls::prepare_call(pool_contract, "maxLiquidityPerTick", &[]).unwrap(),
    ];

    //Collecting arguments from prepared calls for single multicall
    let multicall_args: Vec<DynSolValue> = calls
        .iter()
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

pub async fn fetch_all_bitmaps<P>(
    words: Vec<i16>, //We'll calculate with utils::calculate_bitmap_word_positions
    multicall_contract: &ContractInstance<P>,
    pool_contract: &ContractInstance<P>,
) -> Result<Vec<DynSolValue>, Box<dyn Error>>
where
    P: Provider<Ethereum> + Clone,
{
    let calls: Vec<(Address, Bytes)> = words
        .iter()
        .map(|i| {
            prepare_call(
                pool_contract,
                "tickBitmap",
                &[DynSolValue::Int(utils::i16_to_i256(*i), 256)],
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

pub async fn fetch_all_ticks<P>(
    tick_indices: &[i32],
    multicall_contract: &ContractInstance<P>,
    pool_contract: &ContractInstance<P>,
) -> Result<Vec<DynSolValue>, Box<dyn Error>>
where
    P: Provider<Ethereum> + Clone,
{
    let calls = tick_indices
        .iter()
        .map(|i| {
            prepare_call(
                pool_contract,
                "ticks",
                &[DynSolValue::Int(utils::i32_to_i256(*i), 256)],
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

pub async fn get_pool_data <P: Provider<Ethereum> + Clone>
    (poolcfg: &PoolConfig, 
    abi: &Abi, provider: &P, 
    pool_contract: &ContractInstance<P>, 
    multicall_contract: &ContractInstance<P> ) -> PoolData {

    let initial_data: Vec<DynSolValue> = initial_multicall(multicall_contract, pool_contract)
        .await
        .unwrap()
        .iter()
        .map(|x| {
            match utils::extract_bytes(x) {
                Some(bytes) => utils::decode_response(bytes), // <- Result<DynSolValue, Box<dyn Error>>
                None => Err("Expected bytes but got None".into()),
            }
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()
        .unwrap();


    

    PoolData {
        address: poolcfg.address,
        token0: poolcfg.token0,
        token1: poolcfg.token1,
        fee: poolcfg.fee,
        tick_spacing: ,
        max_liquidity_per_tick: ,
        liquidity: ,
        slot0: ,
        protocol_fees: ,
        ticks:

    }
} //function that returns data that we're intreseted in
