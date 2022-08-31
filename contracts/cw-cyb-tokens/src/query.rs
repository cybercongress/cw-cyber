use cosmwasm_std::{Uint64};
use cid::{Cid, Version};
use std::str::FromStr;
// use cid::multihash::{Code, MultihashDigest};
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
    ticker: String,
    name: String,
    chain_id: String,
    metadata: String,
    denom: Uint64,
    logo: String,
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
    
    for byte in metadata.as_bytes().iter() {
        // "="" && "&" && 0-9 && a-z  (sample string: a=1&b=2&c=3)
        if (*byte != 61) && (*byte != 38) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Metadata. a-z0-9=& allowed".to_string()});
        }
    }

    for byte in chain_id.as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Chain-id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in name.as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Name. a-z0-9- allowed".to_string()});
        }
    }

    for byte in ticker.as_bytes().iter() {
        // 0-9 && A-Z
        if (*byte < 48 || *byte > 57) && (*byte < 65 || *byte > 90) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect ticker. 0-9A-Z allowed".to_string()});
        }
    }



    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        ticker,
        name,
        chain_id,
        metadata,
        denom,
        logo,
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
    name: Option<String>,
    chain_id: Option<String>,
    metadata: Option<String>,
    denom: Option<Uint64>,
    logo: Option<String>,
    // order: Option<Uint64>
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }
    // .unwrap().is_empty()
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

    for byte in metadata.as_ref().unwrap().as_bytes().iter() {
        // "="" && "&" && 0-9 && a-z  (sample string: a=1&b=2&c=3)
        if (*byte != 61) && (*byte != 38) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Metadata. a-z0-9=& allowed".to_string()});
        }
    }

    for byte in chain_id.as_ref().unwrap().as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Chain-id. a-z0-9- allowed".to_string()});
        }
    }

    for byte in name.as_ref().unwrap().as_bytes().iter() {
        // - && 0-9 && a-z
        if (*byte != 45) && (*byte < 48 || *byte > 57) && (*byte < 97 || *byte > 122) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect Name. a-z0-9- allowed".to_string()});
        }
    }

    for byte in ticker.as_ref().unwrap().as_bytes().iter() {
        // 0-9 && A-Z
        if (*byte < 48 || *byte > 57) && (*byte < 65 || *byte > 90) {
            return Err(ContractError::IncorrectInputData {val: "Incorrect ticker. 0-9A-Z allowed".to_string()});
        }
    }
    


    let entry = LIST.load(deps.storage, id)?;
    let updated_entry = Entry {
        id,
        ticker: ticker.unwrap_or(entry.ticker),
        name: name.unwrap_or(entry.name),
        denom: denom.unwrap_or(entry.denom),
        chain_id: chain_id.unwrap_or(entry.chain_id),
        metadata: metadata.unwrap_or(entry.metadata),
        logo: logo.unwrap_or(entry.logo),
        // order: order.unwrap_or(entry.order),
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
