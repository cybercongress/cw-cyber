use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{execute_create_item, execute_delete_entry, execute_update_item, execute_update_owner, query_entry, query_list};
use crate::state::{Config, CONFIG, ENTRY_SEQ};

//@TODO git version iteract
const CONTRACT_NAME: &str = "hub-contracts";
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
            address,
            query_cid,
            execute_cid,
            version,
            chain_id,
            particle,
        } => execute_create_item(deps, info, address, query_cid, execute_cid, version, chain_id, particle),
        ExecuteMsg::UpdateEntry {
            id,
            address,
            query_cid,
            execute_cid,
            version,
            chain_id,
            particle,
        } => execute_update_item(deps, info, id, address,query_cid,execute_cid,version,chain_id, particle),
        ExecuteMsg::DeleteEntry { id } => execute_delete_entry(deps, info, id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEntries { start_after, limit } => {
            to_binary(&query_list(deps, start_after, limit)?)
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