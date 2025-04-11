//Builder  of multicalls
use super::pool_calls::prepare_call;
use crate::utils::{
    calculate_bitmap_word_positions, decode_i32_token, decode_slot0_tokens, decode_ticks,
    decode_u128_token, decode_u256_token, get_initialized_ticks,
};

use crate::types::{PoolConfig, PoolData, TickData};
use std::error::Error;
use web3::{
    contract::Contract,
    ethabi::{Function, Token},
    transports::Http,
    types::{Bytes, CallRequest, U256},
};

pub async fn initial_multicall(
    //Multicall that gets the slot0, tickSpacing, liquidity, maxLiquidityPerTick information of pool
    web3: &web3::Web3<Http>,
    multicall_contract: &Contract<Http>,
    pool_contract: &Contract<Http>,
) -> Result<Vec<Bytes>, Box<dyn Error>> {
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
    web3: &web3::Web3<Http>,
    multicall: &Contract<Http>,
    pool: &Contract<Http>,
) -> Result<Vec<Bytes>, Box<dyn Error>> {
    let calls: Vec<Token> = word_positions
        .iter()
        .map(|&pos| {
            let calldata = pool
                .abi()
                .function("tickBitmap")?
                .encode_input(&[Token::Int(U256::from(pos as u16))])?;
            Ok(Token::Tuple(vec![
                Token::Address(pool.address()),
                Token::Bytes(calldata),
            ]))
        })
        .collect::<Result<Vec<_>, web3::ethabi::Error>>()?;

    let args = Token::Array(calls);
    let call_data = multicall
        .abi()
        .function("aggregate")?
        .encode_input(&[args])?;

    let call_request = CallRequest {
        from: None,
        to: Some(multicall.address()),
        gas: None,
        gas_price: None,
        value: None,
        data: Some(Bytes(call_data)),
        ..Default::default()
    };

    let raw_output: Bytes = web3.eth().call(call_request, None).await?;

    let decoded_tokens = multicall
        .abi()
        .function("aggregate")?
        .decode_output(&raw_output.0)?;

    match decoded_tokens.as_slice() {
        [Token::Uint(_), Token::Array(result_tokens)] => {
            let result = result_tokens
                .iter()
                .map(|t| match t {
                    Token::Bytes(b) => Bytes(b.clone()),
                    _ => panic!("Expected bytes from tickBitmap"),
                })
                .collect();
            Ok(result)
        }
        _ => Err("Invalid output from aggregate".into()),
    }
}

/// Fetches all ticks for the given tick indices using multicall
pub async fn fetch_all_ticks(
    tick_indices: &[i32],
    web3: &web3::Web3<Http>,
    multicall: &Contract<Http>,
    pool: &Contract<Http>,
) -> Result<Vec<Bytes>, Box<dyn Error>> {
    let calls: Vec<Token> = tick_indices
        .iter()
        .map(|&tick| {
            let mut buf = [0u8; 32];
            buf[28..32].copy_from_slice(&(tick as i32).to_be_bytes());
            let calldata = pool
                .abi()
                .function("ticks")?
                .encode_input(&[Token::Int(U256::from_big_endian(&buf))])?;
            Ok(Token::Tuple(vec![
                Token::Address(pool.address()),
                Token::Bytes(calldata),
            ]))
        })
        .collect::<Result<Vec<_>, web3::ethabi::Error>>()?;

    let args = Token::Array(calls);
    let call_data = multicall
        .abi()
        .function("aggregate")?
        .encode_input(&[args])?;

    let call_request = CallRequest {
        from: None,
        to: Some(multicall.address()),
        gas: None,
        gas_price: None,
        value: None,
        data: Some(Bytes(call_data)),
        ..Default::default()
    };

    let raw_output: Bytes = web3.eth().call(call_request, None).await?;

    let decoded_tokens = multicall
        .abi()
        .function("aggregate")?
        .decode_output(&raw_output.0)?;

    match decoded_tokens.as_slice() {
        [Token::Uint(_), Token::Array(result_tokens)] => {
            let result = result_tokens
                .iter()
                .map(|t| match t {
                    Token::Bytes(b) => Bytes(b.clone()),
                    _ => panic!("Expected bytes from ticks"),
                })
                .collect();
            Ok(result)
        }
        _ => Err("Invalid output from aggregate".into()),
    }
}

pub async fn get_pool_data(
    poolcfg: &PoolConfig,
    pool: &Contract<Http>,
    multicall: &Contract<Http>,
    web3: &web3::Web3<Http>,
) -> Result<PoolData, Box<dyn Error>> {
    //Initial multicall (slot0, tickSpacing, liquidity, maxLiquidityPerTick)
    let initial_responses = initial_multicall(web3, multicall, pool).await?;

    //Decode each
    let slot0 = decode_slot0_tokens(pool, "slot0", &initial_responses[0])?;
    let tick_spacing = decode_i32_token(pool, "tickSpacing", &initial_responses[1])?;
    let liquidity = decode_u128_token(pool, "liquidity", &initial_responses[2])?;
    let max_liquidity_per_tick =
        decode_u128_token(pool, "maxLiquidityPerTick", &initial_responses[3])?;

    //Tick bitmap word positions
    let word_positions = calculate_bitmap_word_positions(-887272, 887272, tick_spacing)?;

    //Fetch bitmaps
    let bitmap_responses = fetch_all_bitmaps(&word_positions, web3, multicall, pool).await?;
    let bitmap_u256s = bitmap_responses
        .iter()
        .map(|x| decode_u256_token(pool, "tickBitmap", x))
        .collect::<Result<Vec<U256>, _>>()?;

    //Get all initialized tick indices
    let initialized_tick_indices =
        get_initialized_ticks(&word_positions, &bitmap_u256s, tick_spacing);

    //Fetch tick data
    let ticks_bytes = fetch_all_ticks(&initialized_tick_indices, web3, multicall, pool).await?;
    let ticks: Vec<TickData> = decode_ticks(&initialized_tick_indices, &ticks_bytes, pool)?;

    //Collect everything
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
