Multicall parser of data of Uniswap V3 USDC/USDT pull and tokens 

1.Main starts executing
2.Creating provider instance, based on created rpc_url
3.Parse ABIs of contracts
4.Creating the AppConfig, declared in types.rs, described and created by function in constants.rs
5.Creating Pool Contract instance
6.Using methods declared in pool_calls.rs, we call initial multicall in multicall.rs, that will ask information needed from chain in one single multicall
7.Function-helper helps to calculate bitmaps positions
8.Ask in one multicall about initialization of these bitmaps
9.Ask about specific ticks in one multicall
10.Construct Final PoolData in get_pool_data
11.Print that information