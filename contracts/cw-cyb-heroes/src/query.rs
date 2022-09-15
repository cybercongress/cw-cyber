use cosmwasm_std::{
    Deps, DepsMut, MessageInfo, Order, Response, StdResult, Addr,
};

// use cw_storage_plus::Bound;
use std::ops::Add;

use crate::validating::{validate_by_basic_rule,validate_ipfs_cid};
use crate::error::ContractError;
use crate::msg::{ListResponse};
use crate::state::{Entry, ENTRY_SEQ, items};

const MAX_LIMIT: u32 = 50;
const DEFAULT_LIMIT: u32 = 30;

pub fn uniq_key_by_owner(owner: Addr, id: u64) -> (Addr, u64) {
    (owner.clone(), id.clone())
}

pub fn execute_create_new_item(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
    chain_id: String,
    particle: Option<String>,
) -> Result<Response, ContractError> {

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap(),"particle".to_string());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }


    let validate_address = validate_by_basic_rule(address.clone(), "address".to_string());
    let validate_chain_id = validate_by_basic_rule(chain_id.clone(), "chain_id".to_string());

    if validate_address.is_err() {
        return validate_address;
    }
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }




    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        address,
        chain_id,
        owner: info.sender,
        particle: particle.unwrap_or("".to_string())
    };


    items().save(deps.storage, id, &new_entry)?;
    Ok(Response::new()
        .add_attribute("method", "execute_create_new_item")
        .add_attribute("new_entry_id", id.to_string()))
}

pub fn execute_update_item(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    address: Option<String>,
    chain_id: Option<String>,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    // let owner = CONFIG.load(deps.storage)?.owner;
    // if info.sender != owner {
    //     return Err(ContractError::Unauthorized {});
    // }

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap(),"particle".to_string());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_address = validate_by_basic_rule(address.clone().unwrap(), "address".to_string());
    let validate_chain_id = validate_by_basic_rule(chain_id.clone().unwrap(), "chain_id".to_string());

    if validate_address.is_err() {
        return validate_address;
    }
    if validate_chain_id.is_err() {
        return validate_chain_id;
    }
    

    let entry = items().load(deps.storage, id)?;

    if entry.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    

    let updated_entry = Entry {
        id,
        address: address.unwrap_or(entry.address),
        chain_id: chain_id.unwrap_or(entry.chain_id),
        owner: entry.owner,
        particle: particle.unwrap_or("".to_string()),
    };
    items().save(deps.storage, id, &updated_entry)?;
    Ok(Response::new()
        .add_attribute("method", "execute_update_item")
        .add_attribute("updated_entry_id", id.to_string()))
}

pub fn execute_delete_entry(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    // let owner = CONFIG.load(deps.storage)?.owner;
    // if info.sender != owner {
    //     return Err(ContractError::Unauthorized {});
    // }

    // let key = uniq_key_by_owner(info.sender, id);
    let entry = items().load(deps.storage, id)?;

    if entry.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let _result = items().remove(deps.storage, id);


    Ok(Response::new()
        .add_attribute("method", "execute_delete_entry")
        .add_attribute("deleted_entry_id", id.to_string()))
}





pub fn query_list(deps: Deps, start_after: Option<u64>, limit: Option<u32>, id: Option<u64>, owner: Option<Addr>) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;



    let entries: StdResult<Vec<_>> = items()
        .idx
        .owner
        .prefix(owner.clone().unwrap_or(Addr::unchecked("")).to_string())
        .range(
            deps.storage,            
            None,
            // Some(Bound::exclusive((
            //     owner.clone().unwrap().to_string(),
            //     // start_after.unwrap()
            //     // owner.clone().unwrap().to_string(),
            //     // start_after.unwrap_or_default(),
            // ))),
            None,
            Order::Ascending,
        )
        .take(limit)
        .collect();


    let result = ListResponse {
        entries: entries?.into_iter().map(|l| l.1).collect(),
    };
    Ok(result)
}
