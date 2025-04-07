//Builder  of multicalls
use alloy::contract::Contract;
use super::pool_calls;

pub fn make_multicall_contract(addr: Address, abi: &Abi, provider: &Provider<Http>) -> Contract<Http> {
    Contract::new(addr, &abi.uniswap_pool, provider)
}

pub fn initial_multicall(multicall_contract: &Contract<Http>, pool_contract: &Contract<Http>) -> Result<>{
    let calls: Vec<(Address, Bytes)> = vec![
        pool_calls::prepare_slot0_call(pool_contract),
        pool_calls::prepare_tick_spacing_call(pool_contract),
        pool_calls::prepare_liquidity_call(pool_contract),
        pool_calls::prepare_max_liquidity_per_tick_call(pool_contract),
    ];
    let call = multicall_contract.method::<_, Bytes>("aggregate", (calls))?;
}

pub fn get_pool_data(poolcfg: PoolConfig, app_state: AppState) -> PoolData {

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
