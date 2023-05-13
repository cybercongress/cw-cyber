use std::ops::Add;

use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::state::{CONFIG, Entry, ENTRY_SEQ, LIST};
use crate::validating::{validate_by_basic_rule, validate_ipfs_cid, validate_url};

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

pub fn execute_create_entry(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
    init_particle: String,
    version: String,
    chain_id: String,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;

    if owner.is_none() {
        return Err(ContractError::Unauthorized {});
    } else if info.sender != owner.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap(),"particle".to_string());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_address = validate_by_basic_rule(address.clone(), "address".to_string());
    let validate_init_particle = validate_ipfs_cid(init_particle.clone(), "init_particle".to_string());
    let validate_version = validate_url(version.clone(), "version".to_string());
    let validate_chain_id = validate_url(chain_id.clone(), "chain_id".to_string());

    if validate_address.is_err() {
        return validate_address;
    }
    if validate_init_particle.is_err() {
        return validate_init_particle;
    }
    if validate_version.is_err() {
        return validate_version;
    }
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }

    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        address,
        init_particle,
        version,
        chain_id,
        particle: particle.unwrap_or("".to_string()),
    };

    LIST.save(deps.storage, id, &new_entry)?;

    Ok(Response::new()
        .add_attribute("method", "execute_create_entry")
        .add_attribute("new_entry_id", id.to_string()))
}

pub fn execute_update_entry(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    address: Option<String>,
    init_particle: Option<String>,
    version: Option<String>,
    chain_id: Option<String>,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;

    if owner.is_none() {
        return Err(ContractError::Unauthorized {});
    } else if info.sender != owner.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap(),"particle".to_string());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_address = validate_by_basic_rule(address.clone().unwrap(), "address".to_string());
    let validate_init_particle = validate_ipfs_cid(init_particle.clone().unwrap(), "init_particle".to_string());
    let validate_version = validate_url(version.clone().unwrap(), "version".to_string());
    let validate_chain_id = validate_url(chain_id.clone().unwrap(), "chain_id".to_string());

    if validate_address.is_err() {
        return validate_address;
    }
    if validate_init_particle.is_err() {
        return validate_init_particle;
    }
    if validate_version.is_err() {
        return validate_version;
    }
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }

    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        address: address.unwrap_or(entry.address),
        init_particle: init_particle.unwrap_or(entry.init_particle),
        version: version.unwrap_or(entry.version),
        chain_id: chain_id.unwrap_or(entry.chain_id),
        particle: particle.unwrap_or("".to_string()),
    };

    LIST.save(deps.storage, id, &updated_entry)?;

    Ok(Response::new()
        .add_attribute("method", "execute_update_entry")
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
