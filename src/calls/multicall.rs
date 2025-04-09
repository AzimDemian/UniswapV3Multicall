//Builder  of multicalls

use alloy::{
    contract::{ContractInstance, Interface}, 
    json_abi::JsonAbi, 
    primitives::{Address, U256, Bytes}, 
    providers::RootProvider,
    dyn_abi::DynSolValue,
    sol_types::SolValue
};
use super::pool_calls::{self, prepare_call};
use crate::types::{Abi, PoolConfig, AppConfig, PoolData};



pub fn make_multicall_contract(addr: Address, abi: &Abi, provider: &RootProvider) -> ContractInstance<RootProvider>{ 
    //
    let interface = Interface::new(abi.multicall.clone());
    ContractInstance::new(addr, provider.clone(), interface)
}

pub async fn initial_multicall(multicall_contract: &ContractInstance<RootProvider>, pool_contract: &ContractInstance<RootProvider>) -> Result<(u64, Vec<Bytes>) , Box<dyn Error> >{
    let calls: Vec<(Address, Bytes)> = vec![
        // pool_calls::prepare_slot0_call(pool_contract),
        // pool_calls::prepare_tick_spacing_call(pool_contract),
        // pool_calls::prepare_liquidity_call(pool_contract),
        // pool_calls::prepare_max_liquidity_per_tick_call(pool_contract),
        pool_calls::prepare_call(pool_contract, "slot0", &[]).unwrap(),
        pool_calls::prepare_call(pool_contract, "tickSpacing", &[]).unwrap(),
        pool_calls::prepare_call(pool_contract, "liquidity", &[]).unwrap(),
        pool_calls::prepare_call(pool_contract, "maxLiquidityPerTick", &[]).unwrap(),   
    ];

    let multicall_args: Vec<DynSolValue> = calls.iter()
        .map(|(address, calldata)| {
            DynSolValue::Tuple(vec![
                DynSolValue::Address(*address),
                DynSolValue::Bytes(calldata.clone()),
            ])
        })
        .collect();

    let result = multicall_contract
        .function("aggregate", &[DynSolValue::Array(multicall_args)])?
        .call()
        .await?; 
    
    let (block_number, return_data): (u64, Vec<Bytes>) = result.decode();

    Ok((block_number, return_data));
}

pub async fn fetch_all_bitmaps(words: Vec<i16>, multicall_contract: &ContractInstance<RootProvider>, pool_contract: &ContractInstance<RootProvider>) -> Result<(u64, Vec<Bytes>), Box<dyn Error>>{
    let calls = words.iter().map(|i| {
        prepare_call(pool_contract, "tickBitmap", i);
    }).collect()?;
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
