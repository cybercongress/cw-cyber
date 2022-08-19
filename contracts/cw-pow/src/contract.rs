use std::ops::{Add, Div, Mul, Sub};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256, Uint128, attr, Storage, BankMsg, Coin};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CheckMintSolutionResponse, ExecuteMsg, GetChallengeHashResponse, GetMiningDifficultyResponse, GetMiningRewardResponse, GetMiningTargetResponse, GetMintDigestResponse, InstantiateMsg, QueryMsg};
use crate::state::{SOLUTIONS, State, STATE};
use sha3::{Digest, Keccak256};

const CONTRACT_NAME: &str = "cw-pow";
const CONTRACT_VERSION: &str = "0.1.0";

const BLOCKS_PER_READJUSTMENT: u64 = 128u64;
const TOTAL_SUPPLY: u64= 2100000000000000u64;
const DENOM: &str = "hydrogen";
const MINIMUM_TARGET: Uint256 = Uint256::from_be_bytes([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0,
]); // 2**16 a little number
const MAXIMUM_TARGET: Uint256 = Uint256::from_be_bytes([
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
]); // 2**240 a big number is easier; just find a solution that is smaller, move to 232

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
        latest_difficulty_period_started: env.block.height,
        epoch_count: 0,
        mining_target: Uint256::from_be_bytes([
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]),
        challenge_hash: "cosmos1nfmvw8x37w00p3geuu8lrt3vt5kadxa59730we".to_string(),
        reward_era: 0,
        max_supply_for_era: Uint128::from(TOTAL_SUPPLY.div(2u64)),
        last_reward_to: "".to_string(),
        last_reward_amount: Uint128::zero(),
        last_reward_block_number: 0,
        tokens_mined: Uint128::zero()
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Pow { challenge_digest, nonce } => try_pow(deps, env,info, challenge_digest, nonce)
    }
}

pub fn try_pow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    challenge_digest: String,
    nonce: Uint256,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    let mut hasher = Keccak256::new();

    hasher.update(state.clone().challenge_hash);
    hasher.update(info.clone().sender.to_string());
    hasher.update(nonce.to_string());
    let raw_digest = hasher.finalize();
    let digest = hex::encode(raw_digest.clone());

    if digest.to_string() != challenge_digest {
        return Err(ContractError::WrongDigest {})
    }

    let mut n: [u8; 32] = Default::default();
    n.copy_from_slice(&raw_digest[0..32]);
    let number = Uint256::new(n);

    if number.gt(&state.mining_target) {
        return Err(ContractError::WrongDigest {})
    }

    if SOLUTIONS.has(deps.storage, state.clone().challenge_hash) {
        return Err(ContractError::SolutionExist {})
    }
    SOLUTIONS.save(deps.storage, state.clone().challenge_hash, &digest)?;

    let reward_amount = Uint128::new(5000000000).div(Uint128::new(2).pow(state.reward_era));

    if state.tokens_mined.add(reward_amount).gt(&state.max_supply_for_era) {
        return Err(ContractError::SupplyExceeded {})
    }

    STATE.update(
        deps.storage,
        |mut state| -> Result<State, ContractError> {
            state.tokens_mined = state.tokens_mined.add(reward_amount);
            state.last_reward_to = info.clone().sender.to_string();
            state.last_reward_amount = reward_amount;
            state.last_reward_block_number = env.block.height;
            Ok(state)
        },
    )?;

    _start_new_mining_epoch(
        deps.storage,
        env,
        challenge_digest
    )?;

    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: DENOM.to_string(),
                amount: reward_amount,
            }],
        })
        .add_attributes(vec![
        attr("miner", info.sender.into_string()),
        attr("reward", reward_amount.to_string()),
        attr("epoch", state.epoch_count.to_string()),
        attr("challenge", state.challenge_hash),
    ]))

}

pub fn _start_new_mining_epoch(
    store: &mut dyn Storage,
    env: Env,
    challenge_digest: String,
) -> Result<(), ContractError> {
    let state = STATE.load(store)?;
    let reward_amount = Uint128::new(5000000000).div(Uint128::new(2).pow(state.reward_era));
    if state.tokens_mined.add(reward_amount).gt(&state.max_supply_for_era) && state.reward_era < 39u32 {
        STATE.update(
            store,
            |mut state| -> Result<State, ContractError> {
                state.reward_era = state.reward_era.add(1u32);
                Ok(state)
            },
        )?;
    }

    STATE.update(
        store,
        |mut state| -> Result<State, ContractError> {
            state.max_supply_for_era = Uint128::from(TOTAL_SUPPLY)
                .sub(Uint128::from(TOTAL_SUPPLY)
                    .div(Uint128::from(2u64.pow(state.reward_era.add(1)))));
            state.epoch_count = state.epoch_count.add(1);
            state.challenge_hash = challenge_digest; // TODO change to block hash or rank hash
            Ok(state)
        },
    )?;

    if state.epoch_count % BLOCKS_PER_READJUSTMENT == 0 {
        _readjust_difficulty(store, env)?;
    }

    Ok(())
}


// https://en.bitcoin.it/wiki/Difficulty#What_is_the_formula_for_difficulty
pub fn _readjust_difficulty(
    store: &mut dyn Storage,
    env: Env,
) -> Result<(), ContractError> {
    let state = STATE.load(store)?;
    let blocks_since_last_difficulty_period = env.block.height.sub(state.latest_difficulty_period_started);

    let target_blocks_per_diff_period = BLOCKS_PER_READJUSTMENT.mul(30u64);

    let mut mining_target = state.mining_target;
    if blocks_since_last_difficulty_period.lt(&target_blocks_per_diff_period) {
        let mut excess_block_pct = target_blocks_per_diff_period
            .mul(100u64)
            .div(blocks_since_last_difficulty_period)
            .sub(100u64);
        // make it harder
        if excess_block_pct.gt(&1000u64) {
            excess_block_pct = 1000u64
        }
        mining_target = mining_target.sub(mining_target.div(Uint256::from(2000u64)).mul(Uint256::from(excess_block_pct)))
    } else {
        let mut shortage_block_pct = blocks_since_last_difficulty_period
            .mul(100u64)
            .div(target_blocks_per_diff_period)
            .sub(100u64);
        // make it easier
        if shortage_block_pct.gt(&1000u64) {
            shortage_block_pct = 1000u64
        }
        mining_target = mining_target.sub(mining_target.div(Uint256::from(2000u64)).mul(Uint256::from(shortage_block_pct)))
    }

    if mining_target.lt(&MINIMUM_TARGET) {
        mining_target = MINIMUM_TARGET;
    }
    if mining_target.gt(&MAXIMUM_TARGET) {
        mining_target = MAXIMUM_TARGET;
    }

    STATE.update(
        store,
        |mut state| -> Result<State, ContractError> {
            state.latest_difficulty_period_started = env.block.height;
            state.mining_target = mining_target;
            Ok(state)
        },
    )?;

    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&STATE.load(deps.storage)?),
        QueryMsg::GetChallengeHash {} => to_binary(&query_challgenge_hash(deps)?),
        QueryMsg::GetMiningDifficulty {} => to_binary(&query_mining_difficulty(deps)?),
        QueryMsg::GetMiningTarget {} => to_binary(&query_mining_target(deps)?),
        QueryMsg::GetMiningReward {} => to_binary(&query_mining_reward(deps)?),
        QueryMsg::GetMintDigest {
            challenge_hash,
            address,
            nonce,
        } => to_binary(&query_mint_digest(
            challenge_hash,
            address,
            nonce,
        )?),
        QueryMsg::CheckMintSolution {
            challenge_digest,
            challenge_hash,
            address,
            nonce,
            test_target
        } => to_binary(&query_check_mint_solution(
            challenge_digest,
            challenge_hash,
            address,
            nonce,
            test_target
        )?),

    }
}

fn query_challgenge_hash(deps: Deps) -> StdResult<GetChallengeHashResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetChallengeHashResponse { hash: state.challenge_hash })
}

fn query_mining_difficulty(deps: Deps) -> StdResult<GetMiningDifficultyResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetMiningDifficultyResponse { difficulty: MAXIMUM_TARGET.div(&state.mining_target) })
}

fn query_mining_target(deps: Deps) -> StdResult<GetMiningTargetResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetMiningTargetResponse { target: state.mining_target })
}

fn query_mining_reward(deps: Deps) -> StdResult<GetMiningRewardResponse> {
    let state = STATE.load(deps.storage)?;
    let reward = Uint128::new(5000000000).div(Uint128::new(2).pow(state.reward_era));
    Ok(GetMiningRewardResponse { reward })
}

fn query_mint_digest(
    challenge_hash: String,
    address: String,
    nonce: Uint256,
) -> StdResult<GetMintDigestResponse> {
    let mut hasher = Keccak256::new();
    hasher.update(challenge_hash);
    hasher.update(address);
    hasher.update(nonce.to_string());
    let digest = hex::encode(hasher.finalize());
    Ok(GetMintDigestResponse { digest: digest.to_string() })
}

fn query_check_mint_solution(
    challenge_digest: String,
    challenge_hash: String,
    address: String,
    nonce: Uint256,
    test_target: Uint256
) -> StdResult<CheckMintSolutionResponse> {
    let mut hasher = Keccak256::new();
    hasher.update(challenge_hash);
    hasher.update(address);
    hasher.update(nonce.to_string());

    let raw_digest = hasher.finalize();
    let digest = hex::encode(raw_digest.clone());

    let mut n: [u8; 32] = Default::default();
    n.copy_from_slice(&raw_digest[0..32]);
    let number = Uint256::new(n);
    let pass = digest.eq(&challenge_digest) && number.lt(&test_target);

    Ok(CheckMintSolutionResponse { solution: pass })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn flow() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(2100000000000000, "hydrogen"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMiningDifficulty {}).unwrap();
        let value: GetMiningDifficultyResponse = from_binary(&res).unwrap();
        assert_eq!(Uint256::from(256u64), value.difficulty);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMiningTarget {}).unwrap();
        let value: GetMiningTargetResponse = from_binary(&res).unwrap();
        assert_eq!(Uint256::from_str("6901746346790563787434755862277025452451108972170386555162524223799296").unwrap(), value.target);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::CheckMintSolution {
            challenge_digest: "0000c7a9ccb275cd2d03a2a6abcd00fd7df7bb552cfe4eaf8205e6234ce22ed6".to_string(),
            challenge_hash: "cosmos1nfmvw8x37w00p3geuu8lrt3vt5kadxa59730we".to_string(),
            address: "cosmos1nfmvw8x37w00p3geuu8lrt3vt5kadxa59730we".to_string(),
            nonce: Uint256::from(346874u64),
            test_target: Uint256::from_be_bytes([
                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0,
            ])
        }).unwrap();
        let value: CheckMintSolutionResponse = from_binary(&res).unwrap();
        assert_eq!(true, value.solution);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::CheckMintSolution {
            challenge_digest: "000000f683d7cfb5c7841426c759e9f59389dad4e1663f627d5391a3b37502f4".to_string(),
            challenge_hash: "cosmos1nfmvw8x37w00p3geuu8lrt3vt5kadxa59730we".to_string(),
            address: "cosmos1nfmvw8x37w00p3geuu8lrt3vt5kadxa59730we".to_string(),
            nonce: Uint256::from(17614062u64),
            test_target: Uint256::from_be_bytes([
                0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0,
            ])
        }).unwrap();
        let value: CheckMintSolutionResponse = from_binary(&res).unwrap();
        assert_eq!(true, value.solution);

        let mut env = mock_env();
        env.block.height = env.block.height+128u64;
        let info = mock_info("cosmos1nfmvw8x37w00p3geuu8lrt3vt5kadxa59730we", &[]);
        let msg = ExecuteMsg::Pow {
            challenge_digest: "000000f683d7cfb5c7841426c759e9f59389dad4e1663f627d5391a3b37502f4".to_string(),
            nonce: Uint256::from(17614062u64),
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(true, true);
    }
}
