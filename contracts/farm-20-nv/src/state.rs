use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub distribution_account: Addr,
    pub reward_denom: String,
    pub staking_token: Addr,
    pub lp_token: Option<Addr>,
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    CONFIG.save(storage, config)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub last_distributed: u64,
    pub total_bond_amount: Uint128,
    pub global_reward_index: Decimal,
}

pub const STATE: Item<State> = Item::new("state");

pub fn read_state(storage: &dyn Storage) -> StdResult<State> {
    STATE.load(storage)
}

pub fn store_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    STATE.save(storage, state)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfo {
    pub reward_index: Decimal,
    pub bond_amount: Uint128,
    pub pending_reward: Uint128,
}

pub const STAKERS_INFO: Map<&Addr, StakerInfo> = Map::new("reward");

pub fn store_staker_info(storage: &mut dyn Storage, owner: &Addr, staker_info: &StakerInfo) -> StdResult<()> {
    STAKERS_INFO.save(storage, owner, staker_info)
}

pub fn remove_staker_info(storage: &mut dyn Storage, owner: &Addr) {
    STAKERS_INFO.remove(storage, owner)
}

pub fn read_staker_info(storage: &dyn Storage, owner: &Addr) -> StdResult<StakerInfo> {
    match STAKERS_INFO.may_load(storage, owner)? {
        Some(staker_info) => Ok(staker_info),
        None => Ok(StakerInfo {
            reward_index: Decimal::zero(),
            bond_amount: Uint128::zero(),
            pending_reward: Uint128::zero(),
        }),
    }
}
