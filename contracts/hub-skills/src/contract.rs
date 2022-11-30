use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_list, execute_create_item, execute_update_item, execute_delete_entry, execute_update_owner};
// Token constructor
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
        ExecuteMsg::NewEntry {
            neuron,
            protocol,
            network,
            endpoint,
            particle,
        } => execute_create_item(deps, info, neuron, network, protocol, endpoint, particle),
        ExecuteMsg::UpdateEntry {
            id,
            neuron,
            protocol,
            network,
            endpoint,
            particle,
        } => execute_update_item(deps, info, id, neuron, network, protocol, endpoint, particle),
        ExecuteMsg::DeleteEntry { id } => execute_delete_entry(deps, info, id),
    }
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetItems { start_after, limit, protocol, owner } => {
            to_binary(&query_list(deps, start_after, limit, protocol, owner)?)
        }
    }
}