Multicall parser of data of Uniswap V3 USDC/USDT pull and tokens <br />

1.Main starts executing <br />
2.Creating provider instance, based on created rpc_url <br />
3.Parse ABIs of contracts <br />
4.Creating the AppConfig, declared in types.rs, described and created by function in constants.rs <br />
5.Creating Pool Contract instance <br />
6.Using methods declared in pool_calls.rs, we call initial multicall in multicall.rs, that will ask information needed from chain in one single multicall <br />
7.Function-helper helps to calculate bitmaps positions <br />
8.Ask in one multicall about initialization of these bitmaps <br />
9.Ask about specific ticks in one multicall <br />
10.Construct Final PoolData in get_pool_data <br />
11.Save that information in PoolData.txt
