#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, to_binary};
use cw1155::{IsApprovedForAllResponse, TokenInfoResponse};
use cw2::{get_contract_version, set_contract_version};
use cw_utils::maybe_addr;

use crate::error::ContractError;
use crate::execute::{
    check_can_approve, execute_approve_all, execute_batch_send_from,
    execute_buy, execute_claim, execute_fund, execute_lock, execute_mint,
    execute_revoke_all, execute_sell, execute_send_from, execute_swap_in_out,
    execute_swap_out_in, execute_update_reward
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_all_approvals, query_all_funds, query_all_funds_for_neuron,
   query_all_funds_from_neuron, query_all_neuron_vestings, query_all_tokens,
   query_balance, query_batch_balance, query_spot_price, query_swap_in_out,
   query_swap_out_in, query_token_state, query_tokens
};
use crate::state::{MINTER, TOKENS};
use semver::Version;

const CONTRACT_NAME: &str = "neuron-booster";
const CONTRACT_VERSION: &str = "1.0.0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    MINTER.save(deps.storage, &env.contract.address)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {
            reward,
            locked,
            msg
        } => execute_mint(deps, env, info, reward, locked,msg),
        ExecuteMsg::Fund { token_id } => execute_fund(deps, env, info, token_id),
        ExecuteMsg::Claim { token_id } => execute_claim(deps, env, info, token_id),
        ExecuteMsg::Buy {
            token_id,
            msg
        } => execute_buy(deps, env, info, token_id, msg),
        ExecuteMsg::Sell {
            from,
            token_id,
            value,
        } => execute_sell(deps, env, info, from, token_id, value),
        ExecuteMsg::LockToken { token_id} => execute_lock(deps, env, info, token_id),
        ExecuteMsg::UpdateReward {
            token_id,
            reward
        } => execute_update_reward(deps, env, info, token_id, reward),
        ExecuteMsg::SwapOutIn {
            from,
            to,
            value
        } => execute_swap_out_in(deps, env, info, from, to, value),
        ExecuteMsg::SwapInOut {
            to,
            from,
            value
        } => execute_swap_in_out(deps, env, info, to, from, value),
        ExecuteMsg::SendFrom {
            from,
            to,
            token_id,
            value,
            msg,
        } => execute_send_from(deps, env, info, from, to, token_id, value, msg),
        ExecuteMsg::BatchSendFrom {
            from,
            to,
            batch,
            msg,
        } => execute_batch_send_from(deps, env, info, from, to, batch, msg),
        ExecuteMsg::ApproveAll {
            operator,
            expires
        } => execute_approve_all(deps, env, info, operator, expires),
        ExecuteMsg::RevokeAll { operator } => execute_revoke_all(deps, env, info, operator),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenState { token_id } => {
            to_binary(&query_token_state(deps, env, token_id)?)
        }
        QueryMsg::SpotPrice { token_id } => {
            let token = deps.api.addr_validate(&token_id)?;
            to_binary(&query_spot_price(deps, env, token.to_string())?)
        }
        QueryMsg::FundsByBlock { start_after, limit } => {
            to_binary(&query_all_funds(deps, env, start_after, limit)?)
        },
        QueryMsg::FundsFromNeuron { neuron } => {
            let addr = deps.api.addr_validate(&neuron)?;
            to_binary(&query_all_funds_from_neuron(deps, env, addr)?)
        },
        QueryMsg::FundsForNeuron { token_id } => {
            let token = deps.api.addr_validate(&token_id)?;
            to_binary(&query_all_funds_for_neuron(deps, env, token.to_string())?)
        },
        QueryMsg::Vestings { neuron } => {
            let addr = deps.api.addr_validate(&neuron)?;
            to_binary(&query_all_neuron_vestings(deps, env, addr)?)
        },
        QueryMsg::SwapOutIn { from, to, value } => {
            let token_out = deps.api.addr_validate(&from)?;
            let token_in = deps.api.addr_validate(&to)?;
            to_binary(&query_swap_out_in(deps, env, token_out.to_string(), token_in.to_string(), value)?)
        }
        QueryMsg::SwapInOut { to, from, value } => {
            let token_in = deps.api.addr_validate(&to)?;
            let token_out = deps.api.addr_validate(&from)?;
            to_binary(&query_swap_in_out(deps, env, token_out.to_string(), token_in.to_string(), value)?)
        }
        QueryMsg::Balance { owner, token_id } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            let token = deps.api.addr_validate(&token_id)?;
            to_binary(&query_balance(deps, env, owner_addr, token.to_string())?)
        }
        QueryMsg::BatchBalance { owner, token_ids } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            to_binary(&query_batch_balance(deps, env, owner_addr, token_ids)?)
        }
        QueryMsg::IsApprovedForAll { owner, operator } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            let operator_addr = deps.api.addr_validate(&operator)?;
            let approved = check_can_approve(deps, &env, &owner_addr, &operator_addr)?;
            to_binary(&IsApprovedForAllResponse { approved })
        }
        QueryMsg::ApprovedForAll {
            owner,
            include_expired,
            start_after,
            limit,
        } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            let start_addr = maybe_addr(deps.api, start_after)?;
            to_binary(&query_all_approvals(
                deps,
                env,
                owner_addr,
                include_expired.unwrap_or(false),
                start_addr,
                limit,
            )?)
        }
        QueryMsg::TokenInfo { token_id } => {
            let url = TOKENS.load(deps.storage, &token_id)?;
            to_binary(&TokenInfoResponse { url })
        }
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            to_binary(&query_tokens(deps, owner_addr, start_after, limit)?)
        }
        QueryMsg::AllTokens { start_after, limit } => {
            to_binary(&query_all_tokens(deps, start_after, limit)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: Empty,
) -> Result<Response, ContractError> {
    let stored = get_contract_version(deps.storage)?;
    if stored.contract != CONTRACT_NAME {
        return Err(ContractError::CannotMigrate {
            previous_contract: stored.contract,
        });
    }

    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;

    if storage_version > version {
        return Err(ContractError::CannotMigrateVersion {
            previous_version: stored.version,
        });
    }

    if storage_version < version {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    }

    Ok(Response::new())
}