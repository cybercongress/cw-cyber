// use regex::Regex;
// use cosmwasm_std::{Uint64};

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
    source_chain_id: String,
    destination_chain_id: String,
    source_channel_id: String,
    destination_channel_id: String,
    rpc: String,
    token: String,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    for byte in source_chain_id.as_bytes().iter() {
        //  - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect source_chain_id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in destination_chain_id.as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect destination_chain_id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in source_channel_id.as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect source_channel_id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in destination_channel_id.as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect destination_channel_id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in rpc.as_bytes().iter() {
        // :/.-_0-9a-zA-Z
        if  (*byte != 123) && (*byte != 125) && (*byte != 58) && (*byte != 95) && (*byte != 45) && (*byte != 47) && (*byte != 46) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90)  {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Rpc. Only url allowed".to_string()});
        }
    }

    for byte in token.as_bytes().iter() {
        // 0-9 && A-Z
        if (*byte < 48 || *byte > 57) && (*byte < 65 || *byte > 90) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect token. 0-9A-Z allowed".to_string()});
        }
    }

    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        source_chain_id,
        destination_chain_id,
        source_channel_id,
        destination_channel_id,
        rpc,
        token,
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
    source_chain_id: Option<String>,
    destination_chain_id: Option<String>,
    source_channel_id: Option<String>,
    destination_channel_id: Option<String>,
    rpc: Option<String>,
    token: Option<String>,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    for byte in source_chain_id.as_ref().unwrap().as_bytes().iter() {
        //  - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect source_chain_id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in destination_chain_id.as_ref().unwrap().as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect destination_chain_id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in source_channel_id.as_ref().unwrap().as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect source_channel_id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in destination_channel_id.as_ref().unwrap().as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect destination_channel_id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in rpc.as_ref().unwrap().as_bytes().iter() {
        // :/.-_0-9a-zA-Z
        if  (*byte != 123) && (*byte != 125) &&  (*byte != 58) && (*byte != 95) && (*byte != 45) && (*byte != 47) && (*byte != 46) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) && (*byte < 65 || *byte > 90)  {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Rpc. Only url allowed".to_string()});
        }
    }

    for byte in token.as_ref().unwrap().as_bytes().iter() {
        // 0-9 && A-Z
        if (*byte < 48 || *byte > 57) && (*byte < 65 || *byte > 90) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect token. 0-9A-Z allowed".to_string()});
        }
    }

    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        source_chain_id: source_chain_id.unwrap_or(entry.source_chain_id),
        destination_chain_id: destination_chain_id.unwrap_or(entry.destination_chain_id),
        source_channel_id: source_channel_id.unwrap_or(entry.source_channel_id),
        destination_channel_id: destination_channel_id.unwrap_or(entry.destination_channel_id),
        rpc: rpc.unwrap_or(entry.rpc),
        token: token.unwrap_or(entry.token),
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
