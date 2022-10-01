use cosmwasm_std::{
    Deps, DepsMut, MessageInfo, Order, Response, StdResult,
};

use cw_storage_plus::Bound;
use std::ops::Add;


use crate::error::ContractError;
use crate::msg::{ListResponse};
use crate::state::{Entry, CONFIG, ENTRY_SEQ, LIST};
use crate::validating::{validate_particle,validate_by_basic_rule,validate_url};

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 20;


pub fn execute_create_new_item(
    deps: DepsMut,
    info: MessageInfo,
    data_type: String,
    protocol: String,
    chain_id: String,
    url: String,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }



    let validate_url_result = validate_url(url.clone());
    let validate_particle = validate_particle(particle.clone());
    let validate_data_type = validate_by_basic_rule(data_type.clone(), "data-type".to_string());
    let validate_protocol = validate_by_basic_rule(protocol.clone(), "protocol".to_string());
    let validate_chainid = validate_by_basic_rule(chain_id.clone(), "chain_id".to_string());

    if validate_particle.is_err() {
        return validate_particle;
    }
    if validate_data_type.is_err() {
        return validate_data_type;
    }
    if validate_url_result.is_err() {
        return validate_url_result;
    }
    if validate_protocol.is_err() {
        return validate_protocol;
    }
    if validate_chainid.is_err() {
        return validate_chainid;
    }

    


    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        data_type,
        protocol,
        chain_id,
        url,
        particle: particle.unwrap_or("".to_string()).to_string(),
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
    data_type: String,
    protocol: String,
    chain_id: String,
    url: String,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    let validate_url_result = validate_url(url.clone());
    let validate_particle = validate_particle(particle.clone());
    let validate_data_typer = validate_by_basic_rule(data_type.clone(), "data-type".to_string());
    let validate_protocol = validate_by_basic_rule(protocol.clone(), "protocol".to_string());
    let validate_chainid = validate_by_basic_rule(chain_id.clone(), "chain_id".to_string());

    if validate_particle.is_err() {
        return validate_particle;
    }
    if validate_data_typer.is_err() {
        return validate_data_typer;
    }
    if validate_url_result.is_err() {
        return validate_url_result;
    }
    if validate_protocol.is_err() {
        return validate_protocol;
    }
    if validate_chainid.is_err() {
        return validate_chainid;
    }



    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        data_type,
        protocol,
        chain_id,
        url,
        particle: particle.unwrap_or("".to_string()).to_string(),
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
