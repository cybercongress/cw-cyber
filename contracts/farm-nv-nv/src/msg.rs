use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Decimal, Uint128};
use cw20::Cw20ReceiveMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub distribution_account: String,
    pub reward_denom: String,
    pub staking_denom: String,
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
    pub token_code_id: u64,
    pub pool_name: String,
    pub treasure_account: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Bond {},
    Receive(Cw20ReceiveMsg),
    /// Withdraw pending rewards
    Withdraw {},
    /// Owner operation to stop distribution on current staking contract
    /// and send remaining tokens to the new contract
    MigrateStaking {
        new_staking_contract: String,
    },
    /// Add a list of distribution periods.
    AddDistributionPeriods {
        periods: Vec<(u64, u64, Uint128)>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Unbond {},
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {
        block_height: Option<u64>,
    },
    StakerInfo {
        staker: String,
        block_height: Option<u64>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub distribution_account: String,
    pub reward_denom: String,
    pub staking_denom: String,
    pub lp_token: String,
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
    pub treasure_account: String,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub last_distributed: u64,
    pub total_bond_amount: Uint128,
    pub global_reward_index: Decimal,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfoResponse {
    pub staker: String,
    pub reward_index: Decimal,
    pub bond_amount: Uint128,
    pub pending_reward: Uint128,
}
