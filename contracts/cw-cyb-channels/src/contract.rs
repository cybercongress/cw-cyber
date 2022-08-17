use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_list, execute_create_new_item, execute_update_item, execute_delete_entry};
// Token constructor
use crate::state::{Config, CONFIG, ENTRY_SEQ};

//@TODO git version iteract
const CONTRACT_NAME: &str = "cw-channels";
const CONTRACT_VERSION: &str = "1.0.0";

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
        owner: owner.clone(),
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::NewEntry {
            channel_id,
            network_source,
            network_destination
        } => execute_create_new_item(deps, info, channel_id,network_source,network_destination),
        ExecuteMsg::UpdateEntry {
            id,
            channel_id,
            network_source,
            network_destination,
        } => execute_update_item(deps, info, id, channel_id, network_source, network_destination),
        ExecuteMsg::DeleteEntry { id } => execute_delete_entry(deps, info, id),
    }
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTokens { start_after, limit } => {
            to_binary(&query_list(deps, start_after, limit)?)
        }
    }
}