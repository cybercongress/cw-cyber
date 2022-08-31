use cid::{Cid, Version};
use std::str::FromStr;

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
    name: String,
    chain_id: String,
    genesis_hash: String,
    unbonding_period: String,
    logo: String,
    github: String,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }


    let particle:Cid;
    let try_particle = Cid::from_str(&logo.clone());
    if try_particle.is_ok() {
        particle = try_particle.unwrap();
        if particle.version() != Version::V0 {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Logo".to_string()});
        }
    } else {
        return Err(ContractError::IncorrectInputData {val: "Incorrect Logo".to_string()});
    }

    for byte in chain_id.as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Chain-id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in genesis_hash.as_bytes().iter() {
        // - && 0-9 && a-z 
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Genesis_hash. a-z0-9- allowed".to_string()});
        }
    }

    for byte in name.as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Name. a-z0-9- allowed".to_string()});
        }
    }

    for byte in unbonding_period.as_bytes().iter() {
        // 0-9
        if  *byte < 48 || *byte > 57 {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Unbonding_period. 0-9 allowed".to_string()});
        }
    }

    for byte in github.as_bytes().iter() {
        // :/.-_0-9a-zA-Z
        if  (*byte != 58) && (*byte != 95) && (*byte != 45) && (*byte != 47) && (*byte != 46) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90)  {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Github. Only url allowed".to_string()});
        }
    }


    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;
    //FIXME
    let new_entry = Entry {
        id,
        name,
        chain_id,
        genesis_hash,
        unbonding_period,
        logo,
        github,
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
    name: Option<String>,
    chain_id: Option<String>,
    genesis_hash: Option<String>,
    unbonding_period: Option<String>,
    logo: Option<String>,
    github: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    if !logo.as_ref().is_none() {
        let particle:Cid;
        let try_particle = Cid::from_str(&logo.as_ref().unwrap().clone());
        if try_particle.is_ok() {
            particle = try_particle.unwrap();
            if particle.version() != Version::V0 {
                return Err(ContractError::IncorrectInputData {val: "Incorrect Logo".to_string()});
            }
        } else {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Logo".to_string()});
        }
    }

    for byte in chain_id.as_ref().unwrap().as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Chain-id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in genesis_hash.as_ref().unwrap().as_bytes().iter() {
        // - && 0-9 && a-z 
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Genesis_hash. a-z0-9- allowed".to_string()});
        }
    }

    for byte in name.as_ref().unwrap().as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Name. a-z0-9- allowed".to_string()});
        }
    }

    for byte in unbonding_period.as_ref().unwrap().as_bytes().iter() {
        // 0-9
        if  *byte < 48 || *byte > 57 {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Unbonding_period. 0-9 allowed".to_string()});
        }
    }

    for byte in github.as_ref().unwrap().as_bytes().iter() {
        // :/.-_0-9a-zA-Z
        if  (*byte != 58) && (*byte != 95) && (*byte != 45) && (*byte != 47) && (*byte != 46) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90)  {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Github. Only url allowed".to_string()});
        }
    }

    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        name: name.unwrap_or(entry.name),
        chain_id: chain_id.unwrap_or(entry.chain_id),
        genesis_hash: genesis_hash.unwrap_or(entry.genesis_hash),
        unbonding_period: unbonding_period.unwrap_or(entry.unbonding_period),
        logo: logo.unwrap_or(entry.logo),
        github: github.unwrap_or(entry.github),
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
