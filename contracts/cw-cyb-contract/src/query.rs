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
    address: String,
    query_hash: String,
    execute_hash: String,
    version: String,
    github: String,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    for byte in address.as_bytes().iter() {
        //  0-9 && a-z
        if (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect address. a-z0-9- allowed".to_string()});
        }
    }

    for byte in query_hash.as_bytes().iter() {
        //  0-9 && a-z
        if (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect query_hash. A-Za-z0-9- allowed".to_string()});
        }
    }

    for byte in execute_hash.as_bytes().iter() {
        //  0-9 && a-z
        if (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect execute_hash. A-Za-z0-9- allowed".to_string()});
        }
    }

    for byte in version.as_bytes().iter() {
        // "="" && "&" && 0-9 && a-z  (sample string: a=1&b=2&c=3)
        if (*byte != 61) && (*byte != 38) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect version. a-z0-9=& allowed".to_string()});
        }
    }

    for byte in github.as_bytes().iter() {
        // :/.-_0-9a-zA-Z
        if  (*byte != 58) && (*byte != 95) && (*byte != 45) && (*byte != 47) && (*byte != 46) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90)  {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Github. Only url allowed".to_string()});
        }
    }

    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        address,
        query_hash,
        execute_hash,
        version,
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
    address: Option<String>,
    query_hash: Option<String>,
    execute_hash: Option<String>,
    version: Option<String>,
    github: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    for byte in address.as_ref().unwrap().as_bytes().iter() {
        //  0-9 && a-z
        if (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect address. a-z0-9- allowed".to_string()});
        }
    }

    for byte in query_hash.as_ref().unwrap().as_bytes().iter() {
        //  0-9 && a-z
        if (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect query_hash. A-Za-z0-9- allowed".to_string()});
        }
    }

    for byte in execute_hash.as_ref().unwrap().as_bytes().iter() {
        //  0-9 && a-z
        if (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect execute_hash. A-Za-z0-9- allowed".to_string()});
        }
    }

    for byte in version.as_ref().unwrap().as_bytes().iter() {
        // "="" && "&" && 0-9 && a-z  (sample string: a=1&b=2&c=3)
        if (*byte != 61) && (*byte != 38) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect version. a-z0-9=& allowed".to_string()});
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
        address: address.unwrap_or(entry.address),
        query_hash: query_hash.unwrap_or(entry.query_hash),
        execute_hash: execute_hash.unwrap_or(entry.execute_hash),
        version: version.unwrap_or(entry.version),
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
