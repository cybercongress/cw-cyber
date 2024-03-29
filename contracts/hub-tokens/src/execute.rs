use std::ops::Add;

use cosmwasm_std::{attr, Env, Uint64};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::state::{CONFIG, Entry, ENTRY_SEQ, LIST};
use crate::validating::{validate_by_basic_rule, validate_by_basic_uppercase_rule, validate_ipfs_cid};

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
    ticker: String,
    chain_id: String,
    contract: String,
    decimals: Uint64,
    channel: Uint64,
    logo: String,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;

    if owner.is_none() {
        return Err(ContractError::Unauthorized {});
    } else if info.sender != owner.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    let validate_logo = validate_ipfs_cid(logo.clone(),"logo".to_string());
    if validate_logo.is_err() {
        return validate_logo;
    }

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap(),"particle".to_string());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_ticker = validate_by_basic_uppercase_rule(ticker.clone(), "ticker".to_string());
    let validate_chain_id = validate_by_basic_rule(chain_id.clone(), "chain_id".to_string());

    if validate_ticker.is_err() {
        return validate_ticker;
    }
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }

    // TODO add denom validation
    // TODO add channel validation

    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        ticker,
        chain_id,
        contract,
        decimals,
        channel,
        logo,
        particle: particle.unwrap_or("".to_string())
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
    ticker: Option<String>,
    chain_id: Option<String>,
    contract: Option<String>,
    decimals: Option<Uint64>,
    channel: Option<Uint64>,
    logo: Option<String>,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;

    if owner.is_none() {
        return Err(ContractError::Unauthorized {});
    } else if info.sender != owner.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    let validate_logo = validate_ipfs_cid(logo.clone().unwrap(),"logo".to_string());
    if validate_logo.is_err() {
        return validate_logo;
    }

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap(),"particle".to_string());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_ticker = validate_by_basic_uppercase_rule(ticker.clone().unwrap(), "ticker".to_string());
    let validate_chain_id = validate_by_basic_rule(chain_id.clone().unwrap(), "chain_id".to_string());

    // TODO add denom validation
    // TODO add channel validation

    if validate_ticker.is_err() {
        return validate_ticker;
    }
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }

    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        ticker: ticker.unwrap_or(entry.ticker),
        contract: contract.unwrap_or(entry.contract),
        decimals: decimals.unwrap_or(entry.decimals),
        chain_id: chain_id.unwrap_or(entry.chain_id),
        channel: channel.unwrap_or(entry.channel),
        logo: logo.unwrap_or(entry.logo),
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
