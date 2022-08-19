use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Uint256};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub latest_difficulty_period_started: u64, // block_number
    pub epoch_count: u64, //number of 'blocks' mined
    pub mining_target: Uint256,
    pub challenge_hash: String, //generate a new one when a new reward is minted
    pub reward_era: u32,
    pub max_supply_for_era: Uint128,
    pub last_reward_to: String,
    pub last_reward_amount: Uint128,
    pub last_reward_block_number: u64,
    pub tokens_mined: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
pub const SOLUTIONS: Map<String, String> = Map::new("solutions");
