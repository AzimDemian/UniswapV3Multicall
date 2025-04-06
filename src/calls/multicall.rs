//Builder  of multicall
use alloy::contract::Contract;

pub fn multicall(){

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
