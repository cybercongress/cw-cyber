use std::marker::PhantomData;

use crate::msg::{CyberQueryResponse};

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{to_binary, Binary, Coin, ContractResult, OwnedDeps, SystemResult};
use cyber_std::{CyberQueryWrapper};

/// A drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<CyberQueryWrapper>, CyberQueryWrapper> {
    let custom_querier: MockQuerier<CyberQueryWrapper> =
        MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)])
            .with_custom_handler(|query| SystemResult::Ok(custom_query_execute(query)));
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

pub fn custom_query_execute(query: &CyberQueryWrapper) -> ContractResult<Binary> {
    let _msg = match query {
        &_ => {}
    };
    to_binary(&CyberQueryResponse { msg: "".to_string() }).into()
}

// TODO add tests for querier
