//Builder  of multicalls

use super::pool_calls::prepare_call;
use crate::utils::{
    calculate_bitmap_word_positions, decode_i32_token, decode_slot0_tokens, decode_ticks,
    decode_u128_token, decode_u256_token, get_initialized_ticks,
};

use crate::types::{PoolConfig, PoolData, TickData};
use std::error::Error;
use web3::{
    contract::{Contract, Options},
    ethabi::{Function, Token},
    transports::Http,
    types::{Bytes, CallRequest, U256},
};

pub async fn initial_multicall(
    web3: &web3::Web3<Http>,
    multicall_contract: &Contract<Http>,
    pool_contract: &Contract<Http>,
) -> Result<Vec<Bytes>, Box<dyn Error>> {
    // Создание вызовов к методам контракта пула
    let calls: Vec<Token> = vec![
        prepare_call(pool_contract, "slot0", ()).await?,
        prepare_call(pool_contract, "tickSpacing", ()).await?,
        prepare_call(pool_contract, "liquidity", ()).await?,
        prepare_call(pool_contract, "maxLiquidityPerTick", ()).await?,
    ]
    .into_iter()
    .map(|(addr, data)| Token::Tuple(vec![Token::Address(addr), Token::Bytes(data.0)]))
    .collect();

    let args = vec![Token::Array(calls)];

    // get the func from abi of multicall
    let aggregate_fn: &Function = multicall_contract.abi().function("aggregate")?;

    // encoding of input
    let calldata = aggregate_fn.encode_input(&args)?;

    // Rpc reqwest creation
    let call_request = CallRequest {
        to: Some(multicall_contract.address()),
        data: Some(Bytes(calldata.clone())),
        ..Default::default()
    };

    let raw_output = web3.eth().call(call_request, None).await?;

    // Decode ruesult
    let decoded_tokens = aggregate_fn.decode_output(&raw_output.0)?;

    // Getting the result of call
    if let [Token::Uint(_block), Token::Array(result_tokens)] = decoded_tokens.as_slice() {
        let result_bytes = result_tokens
            .iter()
            .map(|token| match token {
                Token::Bytes(data) => Ok(Bytes::from(data.clone())),
                _ => Err("Expected Token::Bytes".into()),
            })
            .collect::<Result<Vec<Bytes>, Box<dyn Error>>>()?;

        Ok(result_bytes)
    } else {
        Err("Unexpected output format from aggregate".into())
    }
}

pub async fn fetch_all_bitmaps(
    word_positions: &[i16],
    multicall: &Contract<Http>,
    pool: &Contract<Http>,
) -> Result<Vec<Bytes>, Box<dyn Error>> {
    let calls: Vec<Token> = word_positions
        .iter()
        .map(|&pos| {
            let calldata = pool
                .abi()
                .function("tickBitmap")?
                .encode_input(&[Token::Int(pos.into())])?;
            Ok(Token::Tuple(vec![
                Token::Address(pool.address()),
                Token::Bytes(calldata),
            ]))
        })
        .collect::<Result<Vec<_>, web3::ethabi::Error>>()?;

    let args = (Token::Array(calls),);

    let raw_output: Bytes = multicall
        .query("aggregate", args, None, Options::default(), None)
        .await?;

    let decoded_tokens = multicall
        .abi()
        .function("aggregate")?
        .decode_output(&raw_output.0)?;

    if let [Token::Uint(_block), Token::Array(result_tokens)] = decoded_tokens.as_slice() {
        let result: Vec<Bytes> = result_tokens
            .iter()
            .map(|t| match t {
                Token::Bytes(b) => Bytes(b.clone()),
                _ => panic!("Unexpected token format"),
            })
            .collect();
        Ok(result)
    } else {
        Err("Invalid return data from multicall".into())
    }
}

/// Fetches all ticks for the given tick indices using multicall
pub async fn fetch_all_ticks(
    tick_indices: &[i32],
    multicall: &Contract<Http>,
    pool: &Contract<Http>,
) -> Result<Vec<Bytes>, Box<dyn Error>> {
    let calls: Vec<Token> = tick_indices
        .iter()
        .map(|&tick| {
            let calldata = pool
                .abi()
                .function("ticks")?
                .encode_input(&[Token::Int(tick.into())])?;
            Ok(Token::Tuple(vec![
                Token::Address(pool.address()),
                Token::Bytes(calldata),
            ]))
        })
        .collect::<Result<Vec<_>, web3::ethabi::Error>>()?;

    let args = (Token::Array(calls),);

    let raw_output: Bytes = multicall
        .query("aggregate", args, None, Options::default(), None)
        .await?;

    let decoded_tokens = multicall
        .abi()
        .function("aggregate")?
        .decode_output(&raw_output.0)?;

    if let [Token::Uint(_block), Token::Array(result_tokens)] = decoded_tokens.as_slice() {
        let result: Vec<Bytes> = result_tokens
            .iter()
            .map(|t| match t {
                Token::Bytes(b) => Bytes(b.clone()),
                _ => panic!("Unexpected token format"),
            })
            .collect();
        Ok(result)
    } else {
        Err("Invalid return data from multicall".into())
    }
}

pub async fn get_pool_data(
    poolcfg: &PoolConfig,
    pool: &Contract<Http>,
    multicall: &Contract<Http>,
    web3: &web3::Web3<Http>,
) -> Result<PoolData, Box<dyn Error>> {
    // Step 1: Initial multicall (slot0, tickSpacing, liquidity, maxLiquidityPerTick)
    let initial_responses = initial_multicall(web3, multicall, pool).await?;

    // Step 2: Decode each
    let slot0 = decode_slot0_tokens(pool, "slot0", &initial_responses[0])?;
    let tick_spacing = decode_i32_token(pool, "tickSpacing", &initial_responses[1])?;
    let liquidity = decode_u128_token(pool, "liquidity", &initial_responses[2])?;
    let max_liquidity_per_tick =
        decode_u128_token(pool, "maxLiquidityPerTick", &initial_responses[3])?;

    // Step 3: Tick bitmap word positions
    let word_positions = calculate_bitmap_word_positions(-887272, 887272, tick_spacing)?; // this can be dynamic later

    // Step 4: Fetch bitmaps
    let bitmap_responses = fetch_all_bitmaps(&word_positions, multicall, pool).await?;
    let bitmap_u256s = bitmap_responses
        .iter()
        .map(|x| decode_u256_token(pool, "tickBitmap", x))
        .collect::<Result<Vec<U256>, _>>()?;

    // Step 5: Get all initialized tick indices
    let initialized_tick_indices =
        get_initialized_ticks(&word_positions, &bitmap_u256s, tick_spacing);

    // Step 6: Fetch tick data
    let ticks_bytes = fetch_all_ticks(&initialized_tick_indices, multicall, pool).await?;
    let ticks: Vec<TickData> = decode_ticks(&initialized_tick_indices, &ticks_bytes, pool)?;

    // Step 7: Collect everything
    Ok(PoolData {
        address: poolcfg.address,
        token0: poolcfg.token0.clone(),
        token1: poolcfg.token1.clone(),
        fee: poolcfg.fee,
        tick_spacing,
        max_liquidity_per_tick,
        liquidity,
        slot0,
        ticks,
    })
} //function that returns data that we're intreseted in
