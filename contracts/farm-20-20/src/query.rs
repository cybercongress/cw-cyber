use cosmwasm_std::{Deps, StdResult};

use crate::{
    msg::{
        ConfigResponse, StakerInfoResponse, StateResponse,
    },
    state::{
        read_config, read_staker_info, read_state, StakerInfo, State,
    },
    execute::{compute_reward, compute_staker_reward}
};

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let cfg = read_config(deps.storage)?;
    let resp = ConfigResponse {
        distribution_account: cfg.distribution_account.to_string(),
        reward_token: cfg.reward_token.to_string(),
        staking_token: cfg.staking_token.to_string(),
        lp_token: cfg.lp_token.unwrap().to_string(),
        distribution_schedule: cfg.distribution_schedule,
    };

    Ok(resp)
}

pub fn query_state(deps: Deps, block_height: Option<u64>) -> StdResult<StateResponse> {
    let mut state: State = read_state(deps.storage)?;
    if let Some(block_height) = block_height {
        let config = read_config(deps.storage)?;
        compute_reward(&config, &mut state, block_height);
    }

    Ok(StateResponse {
        last_distributed: state.last_distributed,
        total_bond_amount: state.total_bond_amount,
        global_reward_index: state.global_reward_index,
    })
}

pub fn query_staker_info(
    deps: Deps,
    staker: String,
    block_height: Option<u64>,
) -> StdResult<StakerInfoResponse> {
    let staker_addr = deps.api.addr_validate(&staker)?;
    let mut staker_info: StakerInfo = read_staker_info(deps.storage, &staker_addr)?;
    if let Some(block_height) = block_height {
        let config = read_config(deps.storage)?;
        let mut state = read_state(deps.storage)?;

        compute_reward(&config, &mut state, block_height);
        compute_staker_reward(&state, &mut staker_info)?;
    }

    Ok(StakerInfoResponse {
        staker,
        reward_index: staker_info.reward_index,
        bond_amount: staker_info.bond_amount,
        pending_reward: staker_info.pending_reward,
    })
}