use crate::types::Abi;
use alloy::contract::{ContractInstance, Interface};
use alloy::dyn_abi::DynSolValue;
use alloy::network::Ethereum;
use alloy::primitives::{Address, Bytes};
use alloy::providers::Provider;
use std::error::Error;

pub fn make_pool_contract<P: Provider<Ethereum> + Clone>(
    //Creating pool contract (Probably should've made Abi as enum, and then struct that will store abis used in proj,
    // would've been easier later on)
    addr: Address,
    abi: &Abi,
    provider: &impl Provider<Ethereum>,
) -> ContractInstance<impl Provider<Ethereum>> {
    let interf = Interface::new(abi.uniswap_pool.clone());

    ContractInstance::new(addr, provider.clone(), interf)
}

pub fn prepare_call<P: Provider<Ethereum> + Clone>(
    //Function that prepares Eth call to later pass it to multicall
    contract: &ContractInstance<P>,
    method_name: &str,
    args: &[DynSolValue],
) -> Result<(Address, Bytes), Box<dyn Error>> {
    let builder = contract.function(method_name, args)?;
    let calldata = builder.calldata();
    Ok((contract.address().clone(), calldata.clone()))
}
