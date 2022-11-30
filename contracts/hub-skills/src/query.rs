use cosmwasm_std::{
    Deps, DepsMut, MessageInfo, Order, Response, StdResult, Addr,
};

// use cw_storage_plus::Bound;
use std::ops::Add;

use crate::validating::{validate_by_basic_rule,validate_ipfs_cid, validate_url};
use crate::error::ContractError;
use crate::msg::{ListResponse};
use crate::state::{Entry, ENTRY_SEQ, items};

const MAX_LIMIT: u32 = 50;
const DEFAULT_LIMIT: u32 = 30;

pub fn uniq_key_by_owner(owner: Addr, id: u64) -> (Addr, u64) {
    (owner.clone(), id.clone())
}

pub fn execute_create_item(
    deps: DepsMut,
    info: MessageInfo,
    neuron: String,
    network: String,
    protocol: String,
    endpoint: String,
    particle: Option<String>,
) -> Result<Response, ContractError> {

    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap(),"particle".to_string());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_neuron = validate_by_basic_rule(neuron.clone(), "neuron".to_string());
    let validate_network = validate_by_basic_rule(network.clone(), "network".to_string());
    let validate_protocol = validate_by_basic_rule(protocol.clone(), "protocol".to_string());
    let validate_endpoint = validate_url(endpoint.clone(), "endpoint".to_string());

    if validate_neuron.is_err() {
        return validate_neuron;
    }
    if validate_network.is_err() {
        return validate_network;
    }
    if validate_protocol.is_err() {
        return validate_protocol;
    }
    if validate_endpoint.is_err() {
        return validate_endpoint;
    }

    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_entry = Entry {
        id,
        neuron,
        network,
        protocol,
        endpoint,
        owner: info.sender,
        particle: particle.unwrap_or("".to_string())
    };

    items().save(deps.storage, id, &new_entry)?;
    Ok(Response::new()
        .add_attribute("method", "execute_create_item")
        .add_attribute("new_entry_id", id.to_string()))
}

pub fn execute_update_item(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    neuron: Option<String>,
    network: Option<String>,
    protocol: Option<String>,
    endpoint: Option<String>,
    particle: Option<String>,
) -> Result<Response, ContractError> {
    if !particle.as_ref().is_none()  {
        let validate_particle = validate_ipfs_cid(particle.clone().unwrap(),"particle".to_string());
        if validate_particle.is_err() {
            return validate_particle;
        }
    }

    let validate_neuron = validate_by_basic_rule(neuron.clone().unwrap(), "neuron".to_string());
    let validate_network = validate_by_basic_rule(network.clone().unwrap(), "network".to_string());
    let validate_protocol = validate_by_basic_rule(protocol.clone().unwrap(), "protocol".to_string());
    let validate_endpoint = validate_url(endpoint.clone().unwrap(), "endpoint".to_string());

    if validate_neuron.is_err() {
        return validate_neuron;
    }
    if validate_network.is_err() {
        return validate_network;
    }
    if validate_protocol.is_err() {
        return validate_protocol;
    }
    if validate_endpoint.is_err() {
        return validate_endpoint;
    }
    

    let entry = items().load(deps.storage, id)?;

    if entry.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let updated_entry = Entry {
        id,
        neuron: neuron.unwrap_or(entry.neuron),
        network: network.unwrap_or(entry.network),
        protocol: protocol.unwrap_or(entry.protocol),
        endpoint: endpoint.unwrap_or(entry.endpoint),
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
    let entry = items().load(deps.storage, id)?;

    if entry.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let _result = items().remove(deps.storage, id);

    Ok(Response::new()
        .add_attribute("method", "execute_delete_entry")
        .add_attribute("deleted_entry_id", id.to_string()))
}

pub fn query_list(deps: Deps, _start_after: Option<u64>, limit: Option<u32>, _protocol: Option<String>, owner: Option<Addr>) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let indexed_arr = items().idx.owner;

    // TODO add start_after and protocol

    let entries: StdResult<Vec<_>> = indexed_arr
        .prefix(owner.clone().unwrap_or(Addr::unchecked("")).to_string())
        .range(
            deps.storage,            
            None,
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
