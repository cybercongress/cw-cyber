// use regex::Regex;
// use cosmwasm_std::{Uint64};

use cosmwasm_std::{
    Deps, DepsMut, MessageInfo, Order, Response, StdResult,
};

use cw_storage_plus::Bound;
use std::ops::Add;


use crate::validating::{validate_by_basic_rule, validate_ipfs_cid, validate_url};
use crate::error::ContractError;
use crate::msg::{ListResponse};
use crate::state::{Entry, CONFIG, ENTRY_SEQ, LIST};

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 20;


pub fn execute_create_new_item(
    deps: DepsMut,
    info: MessageInfo,
    source_chain_id: String,
    destination_chain_id: String,
    source_channel_id: String,
    destination_channel_id: String,
    explorer_url: String,
    particle: Option<String>
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_source_chain_id = validate_by_basic_rule(source_chain_id.clone(), "source_chain_id".to_string());
    let validate_destination_chain_id = validate_by_basic_rule(destination_chain_id.clone(), "destination_chain_id".to_string());
    let validate_source_channel_id = validate_by_basic_rule(source_channel_id.clone(), "source_channel_id".to_string());
    let validate_destination_channel_id = validate_by_basic_rule(destination_channel_id.clone(), "destination_channel_id".to_string());
    let validate_explorer_url = validate_url(explorer_url.clone(), "explorer_url".to_string());

    if validate_source_chain_id.is_err() {
        return validate_source_chain_id;
    }
    if validate_destination_chain_id.is_err() {
        return validate_destination_chain_id;
    }
    if validate_source_channel_id.is_err() {
        return validate_source_channel_id;
    }
    if validate_destination_channel_id.is_err() {
        return validate_destination_channel_id;
    }
    if validate_explorer_url.is_err() {
        return validate_explorer_url;
    }

    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        source_chain_id,
        destination_chain_id,
        source_channel_id,
        destination_channel_id,
        explorer_url,
        particle: particle.unwrap_or("".to_string()),
    };
    LIST.save(deps.storage, id, &new_entry)?;
    Ok(Response::new()
        .add_attribute("method", "execute_create_item")
        .add_attribute("new_entry_id", id.to_string()))
}

pub fn execute_update_item(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    source_chain_id: Option<String>,
    destination_chain_id: Option<String>,
    source_channel_id: Option<String>,
    destination_channel_id: Option<String>,
    explorer_url: Option<String>,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_source_chain_id = validate_by_basic_rule(source_chain_id.clone().unwrap(), "source_chain_id".to_string());
    let validate_destination_chain_id = validate_by_basic_rule(destination_chain_id.clone().unwrap(), "destination_chain_id".to_string());
    let validate_source_channel_id = validate_by_basic_rule(source_channel_id.clone().unwrap(), "source_channel_id".to_string());
    let validate_destination_channel_id = validate_by_basic_rule(destination_channel_id.clone().unwrap(), "destination_channel_id".to_string());
    let validate_explorer_url = validate_url(explorer_url.clone().unwrap(), "explorer_url".to_string());

    if validate_source_chain_id.is_err() {
        return validate_source_chain_id;
    }
    if validate_destination_chain_id.is_err() {
        return validate_destination_chain_id;
    }
    if validate_source_channel_id.is_err() {
        return validate_source_channel_id;
    }
    if validate_destination_channel_id.is_err() {
        return validate_destination_channel_id;
    }
    if validate_explorer_url.is_err() {
        return validate_explorer_url;
    }

    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        source_chain_id: source_chain_id.unwrap_or(entry.source_chain_id),
        destination_chain_id: destination_chain_id.unwrap_or(entry.destination_chain_id),
        source_channel_id: source_channel_id.unwrap_or(entry.source_channel_id),
        destination_channel_id: destination_channel_id.unwrap_or(entry.destination_channel_id),
        explorer_url: explorer_url.unwrap_or(entry.explorer_url),
        particle: particle.unwrap_or("".to_string()),
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
