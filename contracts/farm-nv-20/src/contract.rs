#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{to_binary, Addr, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg, SubMsg, Reply};

use crate::{
    msg::{
        ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg
    },
    state::{
        read_config, store_config, store_state, Config, State,
    },
    execute::{
        execute_add_distribution_periods, execute_migrate_staking, execute_receive, execute_withdraw,
        execute_change_distribution_account, execute_bond
    },
    query::{
        query_config, query_staker_info, query_state
    },
    error::ContractError
};

use cw20::MinterResponse;
use cw20_base::msg::{InstantiateMsg as Cw20InstantiateMsg};
use cw_utils::parse_reply_instantiate_data;

pub const MSG_REPLY_ID_TOKEN_INSTANT: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    store_config(
        deps.storage,
        &Config {
            distribution_account: deps.api.addr_validate(&msg.distribution_account)?,
            reward_token: deps.api.addr_validate(&msg.reward_token)?,
            staking_denom: msg.staking_denom,
            distribution_schedule: msg.distribution_schedule,
            lp_token: None,
        },
    )?;

    store_state(
        deps.storage,
        &State {
            last_distributed: env.block.height,
            total_bond_amount: Uint128::zero(),
            global_reward_index: Decimal::zero(),
        },
    )?;

    Ok(Response::default()
       .add_submessage(SubMsg::reply_on_success(
           CosmosMsg::Wasm(WasmMsg::Instantiate {
               admin: Some(env.contract.address.to_string()),
               code_id: msg.token_code_id,
               funds: vec![],
               label: "CFv1".to_string(),
               msg: to_binary(&Cw20InstantiateMsg {
                   name: format!("CyberFarm V1 - {}", msg.pool_name),
                   symbol: "CFLST".to_string(),
                   decimals: 0u8,
                   initial_balances: vec![],
                   mint: Some(MinterResponse {
                       minter: env.contract.address.to_string(),
                       cap: None,
                   }),
                   marketing: None
               })?,
           }),
           MSG_REPLY_ID_TOKEN_INSTANT,
   )))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bond {} => execute_bond(deps, env, info),
        ExecuteMsg::Receive(msg) => execute_receive(deps, env, info, msg),
        ExecuteMsg::Withdraw {} => execute_withdraw(deps, env, info),
        ExecuteMsg::AddDistributionPeriods { periods } => {
            execute_add_distribution_periods(deps, env, info, periods)
        }
        ExecuteMsg::MigrateStaking {
            new_staking_contract,
        } => execute_migrate_staking(deps, env, info, new_staking_contract),
        ExecuteMsg::ChangeDistributionAccount { new_account } => {
            execute_change_distribution_account(deps, env, info, new_account)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply
) -> Result<Response, ContractError> {
    match msg.id {
        MSG_REPLY_ID_TOKEN_INSTANT => {
            let address = parse_reply_instantiate_data(msg).unwrap().contract_address;

            let mut config = read_config(deps.storage).unwrap();
            config.lp_token = Some(Addr::unchecked(address.clone()));
            store_config(deps.storage, &config).unwrap();

            Ok(Response::new()
                .add_attribute("reply", "token_instant")
                .add_attribute("token_address", address.to_string()))
        }
        _ => Err(ContractError::InvalidReplyId {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State { block_height } => to_binary(&query_state(deps, block_height)?),
        QueryMsg::StakerInfo {
            staker,
            block_height,
        } => to_binary(&query_staker_info(deps, staker, block_height)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
