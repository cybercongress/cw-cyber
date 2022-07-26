use regex::Regex;
use cosmwasm_std::{Uint64};

use cosmwasm_std::{
    Deps, DepsMut, MessageInfo, Order, Response, StdResult,
};

use cw_storage_plus::Bound;
use std::ops::Add;


use crate::error::ContractError;
use crate::msg::{ListResponse};
use crate::state::{Entry, CONFIG, ENTRY_SEQ, LIST};

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 20;


pub fn execute_create_new_item(
    deps: DepsMut,
    info: MessageInfo,
    ticker: String,
    name: String,
    denom: Uint64,
    logo: String,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }
    let _base_field_expression = Regex::new("[^a-zA-Z0-9]$").unwrap();

    if _base_field_expression.is_match(&ticker) {
        return Err(ContractError::IncorrectInputData {});
    }

    if _base_field_expression.is_match(&name) {
        return Err(ContractError::IncorrectInputData {});
    }

    if _base_field_expression.is_match(&logo) {
        return Err(ContractError::IncorrectInputData {});
    }

    
    // assert!(baseFieldExp.is_match(ticker));
    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;
    //FIXME
    let new_entry = Entry {
        id,
        ticker,
        name,
        denom,
        logo,
    };
    LIST.save(deps.storage, id, &new_entry)?;
    Ok(Response::new()
        .add_attribute("method", "execute_create_new_item")
        .add_attribute("new_entry_id", id.to_string()))
}

pub fn execute_update_item(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    ticker: Option<String>,
    name: Option<String>,
    denom: Option<Uint64>,
    logo: Option<String>,
    // order: Option<Uint64>
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }
    let _base_field_expression = Regex::new("[^a-zA-Z0-9]$").unwrap();

    if _base_field_expression.is_match(&ticker.as_ref().unwrap()) {
        return Err(ContractError::IncorrectInputData {});
    }

    if _base_field_expression.is_match(&name.as_ref().unwrap()) {
        return Err(ContractError::IncorrectInputData {});
    }

    if _base_field_expression.is_match(&logo.as_ref().unwrap()) {
        return Err(ContractError::IncorrectInputData {});
    }

    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        ticker: ticker.unwrap_or(entry.ticker),
        name: name.unwrap_or(entry.name),
        denom: denom.unwrap_or(entry.denom),
        logo: logo.unwrap_or(entry.logo),
        // order: order.unwrap_or(entry.order),
    };
    LIST.save(deps.storage, id, &updated_entry)?;
    Ok(Response::new()
        .add_attribute("method", "execute_update_item")
        .add_attribute("updated_entry_id", id.to_string()))
}

pub fn execute_delete_entry(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    LIST.remove(deps.storage, id);
    Ok(Response::new()
        .add_attribute("method", "execute_delete_entry")
        .add_attribute("deleted_entry_id", id.to_string()))
}



// fn query_entry(deps: Deps, id: u64) -> StdResult<EntryResponse> {
//     let entry = LIST.load(deps.storage, id)?;
//     Ok(EntryResponse {
//         id: entry.id,
//         description: entry.description,
//         status: entry.status,
//         priority: entry.priority,
//     })
// }


pub fn query_list(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let entries: StdResult<Vec<_>> = LIST
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect();

    let result = ListResponse {
        entries: entries?.into_iter().map(|l| l.1).collect(),
    };
    Ok(result)
}
