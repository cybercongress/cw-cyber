use cosmwasm_std::{Uint64};
// use cid::{Cid, Version};
// use std::str::FromStr;
// use cid::multihash::{Code, MultihashDigest};
use cosmwasm_std::{
    Deps, DepsMut, MessageInfo, Order, Response, StdResult,
};

use cw_storage_plus::Bound;
use std::ops::Add;

use crate::validating::{validate_by_basic_rule,validate_by_basic_uppercase_rule,validate_ipfs_cid,validate_by_int_range};
use crate::error::ContractError;
use crate::msg::{ListResponse};
use crate::state::{Entry, CONFIG, ENTRY_SEQ, LIST};

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 20;


pub fn execute_create_new_item(
    deps: DepsMut,
    info: MessageInfo,
    ticker: String,
    chain_id: String,
    denom: Uint64,
    logo: String,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
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
    let validate_denom = validate_by_int_range(Uint64::from(1u64),Uint64::from(18u64), denom.clone(),"denom".to_string());

    if validate_ticker.is_err() {
        return validate_ticker;
    }
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }
    if validate_denom.is_err() {
        return validate_denom;
    }



    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        ticker,
        chain_id,
        denom,
        logo,
        particle: particle.unwrap_or("".to_string())
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
    chain_id: Option<String>,
    denom: Option<Uint64>,
    logo: Option<String>,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
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
    let validate_denom = validate_by_int_range(Uint64::from(1u64),Uint64::from(18u64), denom.clone().unwrap(),"denom".to_string());

    if validate_ticker.is_err() {
        return validate_ticker;
    }
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }
    if validate_denom.is_err() {
        return validate_denom;
    } 


    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        ticker: ticker.unwrap_or(entry.ticker),
        denom: denom.unwrap_or(entry.denom),
        chain_id: chain_id.unwrap_or(entry.chain_id),
        logo: logo.unwrap_or(entry.logo),
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
