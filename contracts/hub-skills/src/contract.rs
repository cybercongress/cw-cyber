use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::execute::{execute_create_entry, execute_delete_entry, execute_update_entry, execute_update_entry_owner, execute_update_owner};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{query_entry, query_list_by_network, query_list_by_owner, query_list_by_protocol};
use crate::state::{Config, CONFIG, ENTRY_SEQ};

//@TODO git version iteract
const CONTRACT_NAME: &str = "cw-skills";
const CONTRACT_VERSION: &str = "0.1.0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .owner
        .and_then(|addr_string| deps.api.addr_validate(addr_string.as_str()).ok())
        .unwrap_or(info.sender);

    let config = Config {
        owner: Some(owner.clone()),
    };

    CONFIG.save(deps.storage, &config)?;

    ENTRY_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwner { new_owner } => execute_update_owner(deps, env, info, new_owner),
        ExecuteMsg::UpdateEntryOwner { id, new_owner } => execute_update_entry_owner(deps, env, info, id, new_owner),
        ExecuteMsg::CreateEntry {
            neuron,
            protocol,
            network,
            endpoint,
            particle,
        } => execute_create_entry(deps, info, neuron, network, protocol, endpoint, particle),
        ExecuteMsg::UpdateEntry {
            id,
            neuron,
            protocol,
            network,
            endpoint,
            particle,
        } => execute_update_entry(deps, info, id, neuron, network, protocol, endpoint, particle),
        ExecuteMsg::DeleteEntry { id } => execute_delete_entry(deps, info, id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEntries { start_after, limit, owner } => {
            to_binary(&query_list_by_owner(deps, start_after, limit, owner)?)
        }
        QueryMsg::GetEntriesProtocol { start_after, limit, protocol } => {
            to_binary(&query_list_by_protocol(deps, start_after, limit, protocol)?)
        }
        QueryMsg::GetEntriesNetwork { start_after, limit, network } => {
            to_binary(&query_list_by_network(deps, start_after, limit, network)?)
        }
        QueryMsg::GetEntry { id } => {
            to_binary(&query_entry(deps, id)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let version = get_contract_version(deps.storage)?;
    if version.contract != CONTRACT_NAME {
        return Err(ContractError::CannotMigrate {
            previous_contract: version.contract,
        });
    }

    Ok(Response::default())
}