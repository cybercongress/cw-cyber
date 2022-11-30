use cosmwasm_std::{attr, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult};

use cw_storage_plus::Bound;
use std::ops::Add;


use crate::error::ContractError;
use crate::msg::{ListResponse};
use crate::state::{Entry, CONFIG, ENTRY_SEQ, LIST};
use crate::validating::{validate_particle,validate_datatype};

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 20;

pub fn execute_update_owner(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_owner: Option<String>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    let owner = cfg.owner.ok_or(ContractError::Unauthorized {})?;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    let mut tmp_owner = None;
    if let Some(addr) = new_owner {
        tmp_owner = Some(deps.api.addr_validate(&addr)?)
    }

    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
        exists.owner = tmp_owner;
        Ok(exists)
    })?;

    Ok(Response::new().add_attributes(vec![attr("action", "update_owner")]))
}

pub fn execute_create_new_item(
    deps: DepsMut,
    info: MessageInfo,
    data_type: String,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;

    if owner.is_none() {
        return Err(ContractError::Unauthorized {});
    } else if info.sender != owner.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    let validate_particle = validate_particle(particle.clone());
    let validate_data_typer = validate_datatype(data_type.clone());

    if validate_particle.is_err() {
        return validate_particle;
    }
    if validate_data_typer.is_err() {
        return validate_data_typer;
    }



    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        data_type: data_type,
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
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;

    if owner.is_none() {
        return Err(ContractError::Unauthorized {});
    } else if info.sender != owner.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    let validate_particle = validate_particle(particle.clone());
    let validate_data_typer = validate_datatype(data_type.clone());

    if validate_particle.is_err() {
        return validate_particle;
    }
    if validate_data_typer.is_err() {
        return validate_data_typer;
    }

    let _entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        data_type: data_type,
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

    if owner.is_none() {
        return Err(ContractError::Unauthorized {});
    } else if info.sender != owner.unwrap() {
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
