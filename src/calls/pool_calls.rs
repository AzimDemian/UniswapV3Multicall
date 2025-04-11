use std::error::Error;
use web3::contract::Contract;
use web3::contract::tokens::Tokenize;
use web3::types::{Address, Bytes};

// Создаёт контракт пула из ABI и адреса

pub async fn prepare_call<T: Tokenize>(
    contract: &Contract<web3::transports::Http>,
    method: &str,
    args: T,
) -> Result<(Address, Bytes), Box<dyn Error>> {
    let function = contract.abi().function(method)?;
    let calldata = function.encode_input(&args.into_tokens())?;
    Ok((contract.address(), Bytes(calldata)))
}
