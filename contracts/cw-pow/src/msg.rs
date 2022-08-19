use cosmwasm_std::{Uint128, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Pow {
        challenge_digest: String,
        nonce: Uint256,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetState {},
    GetChallengeHash {},
    GetMiningDifficulty {},
    GetMiningTarget {},
    GetMiningReward {},
    // debug
    GetMintDigest {
        challenge_hash: String,
        address: String,
        nonce: Uint256,
    },
    // debug
    CheckMintSolution {
        challenge_digest: String,
        challenge_hash: String,
        address: String,
        nonce: Uint256,
        test_target: Uint256
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetChallengeHashResponse {
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetMiningDifficultyResponse {
    pub difficulty: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetMiningTargetResponse {
    pub target: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetMiningRewardResponse {
    pub reward: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetMintDigestResponse {
    pub digest: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CheckMintSolutionResponse {
    pub solution: bool,
}

