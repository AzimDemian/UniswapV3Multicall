use alloy::contract::Contract;
use alloy::primitives::Address;

pub fn make_pool_contract(addr: Address, abi: &Abi, provider: &Provider<Http>) -> Contract<Http> {
    Contract::new(addr, abi, provider)
}

pub fn prepare_slot0_call(contract: &Contract<Http>) -> Result<(Address, Bytes), Error> {
    let call = contract.method::<_, Bytes>("slot0", ())?;
    Ok((contract.address(), call.calldata()?))
}

pub fn prepare_