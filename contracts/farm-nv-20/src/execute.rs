use cosmwasm_std::{from_binary, to_binary, Addr, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg, BankMsg, Coin};

use crate::{
    msg::{
        Cw20HookMsg
    },
    state::{
        read_config, read_staker_info, read_state, remove_staker_info, store_config,
        store_staker_info, store_state, Config, StakerInfo, State,
    },
};

use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use crate::error::ContractError;

pub fn execute_add_distribution_periods(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    periods: Vec<(u64, u64, Uint128)>,
) -> Result<Response, ContractError> {
    let mut config: Config = read_config(deps.storage)?;

    if info.sender != config.distribution_account {
        return Err(ContractError::Unauthorized {});
    }

    config.distribution_schedule.extend(periods);
    store_config(deps.storage, &config)?;

    Ok(Response::default())
}

pub fn execute_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let config: Config = read_config(deps.storage)?;

    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Unbond {}) => {
            if config.lp_token.unwrap() != info.sender {
                return Err(ContractError::Unauthorized {});
            }

            let cw20_sender = deps.api.addr_validate(&cw20_msg.sender)?;
            execute_unbond(deps, env, cw20_sender, cw20_msg.amount)
        }
        Err(_) => Err(ContractError::Unauthorized {}),
    }
}

pub fn execute_bond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo
) -> Result<Response, ContractError> {
    let sender_addr_raw: Addr = deps.api.addr_validate(info.sender.as_str())?;

    let config: Config = read_config(deps.storage)?;

    let mut amount:Uint128 = Uint128::new(0);
    info.funds.iter().for_each(|fund| {
        if fund.denom == config.staking_denom {
            amount = fund.amount;
        }
    });

    if amount == Uint128::new(0) {
        return Err(ContractError::FailBond {});
    }

    let mut state: State = read_state(deps.storage)?;
    let mut staker_info: StakerInfo = read_staker_info(deps.storage, &sender_addr_raw)?;

    // Compute global reward & staker reward
    compute_reward(&config, &mut state, env.block.height);
    compute_staker_reward(&state, &mut staker_info)?;

    // Increase bond_amount
    increase_bond_amount(&mut state, &mut staker_info, amount);

    // Store updated state with staker's staker_info
    store_staker_info(deps.storage, &sender_addr_raw, &staker_info)?;
    store_state(deps.storage, &state)?;

    Ok(Response::new()
        .add_attributes(vec![
            ("action", "bond"),
            ("owner", info.sender.as_str()),
            ("amount", amount.to_string().as_str()),
        ])
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.lp_token.unwrap().to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Mint {
                recipient: info.sender.to_string(),
                amount: amount,
            })?,
            funds: vec![],
        }))
    )
}

pub fn execute_unbond(
    deps: DepsMut,
    env: Env,
    sender_addr: Addr,
    amount: Uint128
) -> Result<Response, ContractError> {
    let config: Config = read_config(deps.storage)?;
    let sender_addr_raw: Addr = deps.api.addr_validate(sender_addr.clone().as_str())?;

    let mut state: State = read_state(deps.storage)?;
    let mut staker_info: StakerInfo = read_staker_info(deps.storage, &sender_addr_raw)?;

    if staker_info.bond_amount < amount {
        return Err(ContractError::FailUnbond {});
    }

    // Compute global reward & staker reward
    compute_reward(&config, &mut state, env.block.height);
    compute_staker_reward(&state, &mut staker_info)?;

    // Decrease bond_amount
    decrease_bond_amount(&mut state, &mut staker_info, amount)?;

    // Store or remove updated rewards info
    // depends on the left pending reward and bond amount
    if staker_info.pending_reward.is_zero() && staker_info.bond_amount.is_zero() {
        remove_staker_info(deps.storage, &sender_addr_raw);
    } else {
        store_staker_info(deps.storage, &sender_addr_raw, &staker_info)?;
    }

    // Store updated state
    store_state(deps.storage, &state)?;

    Ok(Response::new()
        .add_messages(vec![
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: config.lp_token.unwrap().to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Burn {
                    amount,
                })?,
                funds: vec![],
            }),
            CosmosMsg::Bank(BankMsg::Send {
                to_address: sender_addr.to_string(),
                amount: vec![Coin {
                    denom: config.staking_denom.to_string(),
                    amount,
                }],
            }),
        ])
        .add_attributes(vec![
            ("action", "unbond"),
            ("owner", sender_addr.to_string().as_str()),
            ("amount", amount.to_string().as_str()),
        ]))
}

// withdraw rewards to executor
pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo
) -> Result<Response, ContractError> {
    let config: Config = read_config(deps.storage)?;
    let mut state: State = read_state(deps.storage)?;
    let mut staker_info = read_staker_info(deps.storage, &info.sender.clone())?;

    // Compute global reward & staker reward
    compute_reward(&config, &mut state, env.block.height);
    compute_staker_reward(&state, &mut staker_info)?;

    let amount = staker_info.pending_reward;
    staker_info.pending_reward = Uint128::zero();

    // Store or remove updated rewards info
    // depends on the left pending reward and bond amount
    if staker_info.bond_amount.is_zero() {
        remove_staker_info(deps.storage, &info.sender);
    } else {
        store_staker_info(deps.storage, &info.sender, &staker_info)?;
    }

    // Store updated state
    store_state(deps.storage, &state)?;

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.reward_token.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount,
            })?,
            funds: vec![],
        }))
        .add_attributes(vec![
            ("action", "withdraw"),
            ("owner", info.sender.as_str()),
            ("amount", amount.to_string().as_str()),
        ]))
}

pub fn execute_migrate_staking(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_staking_contract: String,
) -> Result<Response, ContractError> {
    let mut config: Config = read_config(deps.storage)?;
    let mut state: State = read_state(deps.storage)?;

    if info.sender != config.distribution_account {
        return Err(ContractError::Unauthorized {});
    }

    // compute global reward, sets last_distributed_height to env.block.height
    compute_reward(&config, &mut state, env.block.height);

    let total_distribution_amount: Uint128 =
        config.distribution_schedule.iter().map(|item| item.2).sum();

    let block_height = env.block.height;
    // eliminate distribution slots that have not started
    config
        .distribution_schedule
        .retain(|slot| slot.0 < block_height);

    let mut distributed_amount = Uint128::zero();
    for s in config.distribution_schedule.iter_mut() {
        if s.1 < block_height {
            // all distributed
            distributed_amount += s.2;
        } else {
            // partially distributed slot
            let num_blocks = s.1 - s.0;
            let distribution_amount_per_block: Decimal = Decimal::from_ratio(s.2, num_blocks);

            let passed_blocks = block_height - s.0;
            let distributed_amount_on_slot =
                distribution_amount_per_block * Uint128::from(passed_blocks as u128);
            distributed_amount += distributed_amount_on_slot;

            // modify distribution slot
            s.1 = block_height;
            s.2 = distributed_amount_on_slot;
        }
    }

    // update config
    store_config(deps.storage, &config)?;
    // update state
    store_state(deps.storage, &state)?;

    let remaining_anc = total_distribution_amount.checked_sub(distributed_amount)?;

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.reward_token.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: new_staking_contract,
                amount: remaining_anc,
            })?,
            funds: vec![],
        }))
        .add_attributes(vec![
            ("action", "migrate_staking"),
            ("distributed_amount", &distributed_amount.to_string()),
            ("remaining_amount", &remaining_anc.to_string()),
        ]))
}

pub fn execute_change_distribution_account(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_account: String,
) -> Result<Response, ContractError> {
    let mut config: Config = read_config(deps.storage)?;

    if info.sender != config.distribution_account {
        return Err(ContractError::Unauthorized {});
    }

    config.distribution_account = deps.api.addr_validate(&new_account)?;
    store_config(deps.storage, &config)?;

    Ok(Response::default())
}

fn increase_bond_amount(state: &mut State, staker_info: &mut StakerInfo, amount: Uint128) {
    state.total_bond_amount += amount;
    staker_info.bond_amount += amount;
}

fn decrease_bond_amount(
    state: &mut State,
    staker_info: &mut StakerInfo,
    amount: Uint128,
) -> StdResult<()> {
    state.total_bond_amount = state.total_bond_amount.checked_sub(amount)?;
    staker_info.bond_amount = staker_info.bond_amount.checked_sub(amount)?;
    Ok(())
}

// compute distributed rewards and update global reward index
pub fn compute_reward(
    config: &Config,
    state: &mut State,
    block_height: u64
) {
    if state.total_bond_amount.is_zero() {
        state.last_distributed = block_height;
        return;
    }

    let mut distributed_amount: Uint128 = Uint128::zero();
    for s in config.distribution_schedule.iter() {
        if s.0 > block_height || s.1 < state.last_distributed {
            continue;
        }

        // min(s.1, block_height) - max(s.0, last_distributed)
        let passed_blocks =
            std::cmp::min(s.1, block_height) - std::cmp::max(s.0, state.last_distributed);

        let num_blocks = s.1 - s.0;
        let distribution_amount_per_block: Decimal = Decimal::from_ratio(s.2, num_blocks);
        distributed_amount += distribution_amount_per_block * Uint128::from(passed_blocks as u128);
    }

    state.last_distributed = block_height;
    state.global_reward_index = state.global_reward_index
        + Decimal::from_ratio(distributed_amount, state.total_bond_amount);
}

// withdraw reward to pending reward
pub fn compute_staker_reward(
    state: &State,
    staker_info: &mut StakerInfo
) -> StdResult<()> {
    let pending_reward = (staker_info.bond_amount * state.global_reward_index)
        .checked_sub(staker_info.bond_amount * staker_info.reward_index)?;

    staker_info.reward_index = state.global_reward_index;
    staker_info.pending_reward += pending_reward;
    Ok(())
}