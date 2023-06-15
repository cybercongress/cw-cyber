use std::ops::Add;

use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::state::{CONFIG, Entry, ENTRY_SEQ, LIST};
use crate::validating::{validate_by_basic_rule, validate_chain_id, validate_ipfs_cid, validate_period};

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
    name: String,
    chain_id: String,
    prefix: String,
    genesis_hash: String,
    protocol: String,
    unbonding_period: String,
    logo: String,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;

    if owner.is_none() {
        return Err(ContractError::Unauthorized {});
    } else if info.sender != owner.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    let validate_genesis_hash = validate_ipfs_cid(genesis_hash.clone());
    if validate_genesis_hash.is_err() {
        return validate_genesis_hash;
    }

    let validate_logo = validate_ipfs_cid(logo.clone());
    if validate_logo.is_err() {
        return validate_logo;
    }

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_chain_id = validate_chain_id(chain_id.clone(), "chain-id".to_string());
    let validate_prefix = validate_by_basic_rule(prefix.clone(), "prefix".to_string());
    let validate_name = validate_by_basic_rule(name.clone(), "name".to_string());
    let validate_protocol = validate_by_basic_rule(protocol.clone(), "protocol".to_string());
    let validate_unbonding_period = validate_period(unbonding_period.clone(), "protocol".to_string());

    if validate_chain_id.is_err() {
        return validate_chain_id;
    }
    if validate_prefix.is_err() {
        return validate_prefix;
    }
    if validate_genesis_hash.is_err() {
        return validate_genesis_hash;
    }
    if validate_name.is_err() {
        return validate_name;
    }
    if validate_protocol.is_err() {
        return validate_protocol;
    }
    if validate_unbonding_period.is_err() {
        return validate_protocol;
    }

    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        name,
        chain_id,
        prefix,
        genesis_hash,
        protocol,
        unbonding_period,
        logo,
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
    name: Option<String>,
    chain_id: Option<String>,
    prefix: Option<String>,
    genesis_hash: Option<String>,
    protocol: Option<String>,
    unbonding_period: Option<String>,
    logo: Option<String>,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;

    if owner.is_none() {
        return Err(ContractError::Unauthorized {});
    } else if info.sender != owner.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    let validate_genesis_hash = validate_ipfs_cid(genesis_hash.clone().unwrap());
    if validate_genesis_hash.is_err() {
        return validate_genesis_hash;
    }

    let validate_logo = validate_ipfs_cid(logo.clone().unwrap());
    if validate_logo.is_err() {
        return validate_logo;
    }

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_chain_id = validate_chain_id(chain_id.as_ref().unwrap().clone(), "chain-id".to_string());
    let validate_prefix = validate_by_basic_rule(prefix.as_ref().unwrap().clone(), "prefix".to_string());
    let validate_name = validate_by_basic_rule(name.as_ref().unwrap().clone(), "name".to_string());
    let validate_protocol = validate_by_basic_rule(protocol.as_ref().unwrap().clone(), "protocol".to_string());
    let validate_unbonding_period = validate_period(unbonding_period.as_ref().unwrap().clone(), "protocol".to_string());

    if validate_chain_id.is_err() {
        return validate_chain_id;
    }
    if validate_prefix.is_err() {
        return validate_prefix;
    }
    if validate_genesis_hash.is_err() {
        return validate_genesis_hash;
    }
    if validate_name.is_err() {
        return validate_name;
    }
    if validate_protocol.is_err() {
        return validate_protocol;
    }
    if validate_unbonding_period.is_err() {
        return validate_protocol;
    }

    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        name: name.unwrap_or(entry.name),
        chain_id: chain_id.unwrap_or(entry.chain_id),
        prefix: prefix.unwrap_or(entry.prefix),
        genesis_hash: genesis_hash.unwrap_or(entry.genesis_hash),
        protocol: protocol.unwrap_or(entry.protocol),
        unbonding_period: unbonding_period.unwrap_or(entry.unbonding_period),
        logo: logo.unwrap_or(entry.logo),
        particle: particle.unwrap_or(entry.particle),
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
