use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Binary, Decimal, Uint128};
use cw_utils::Expiration;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

pub type TokenId = String;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint {
        reward: u64,
        locked: bool,
        msg: Option<Binary>
    },
    Fund { token_id: TokenId },
    Claim { token_id: TokenId },
    Buy { token_id: TokenId, msg: Option<Binary> },
    Sell {
        from: String,
        token_id: TokenId,
        value: Uint128,
    },
    LockToken { token_id: TokenId },
    UpdateReward { token_id: TokenId, reward: u64 },
    SwapOutIn {
        from: TokenId,
        to: TokenId,
        value: Uint128
    },
    SwapInOut {
        to: TokenId,
        from: TokenId,
        value: Uint128
    },
    SendFrom {
        from: String,
        to: String,
        token_id: TokenId,
        value: Uint128,
        msg: Option<Binary>,
    },
    BatchSendFrom {
        from: String,
        to: String,
        batch: Vec<(TokenId, Uint128)>,
        msg: Option<Binary>,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll { operator: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenState { token_id: TokenId },
    SpotPrice { token_id: TokenId },
    FundsByBlock {
        start_after: Option<u64>,
        limit: Option<u32>
    },
    FundsFromNeuron {
        // TODO add pagination
        neuron: String,
    },
    FundsForNeuron {
        // TODO add pagination
        token_id: TokenId,
    },
    Vestings {
        // TODO add pagination
        neuron: String,
    },
    SwapOutIn {
        from: TokenId,
        to: TokenId,
        value: Uint128
    },
    SwapInOut {
        to: TokenId,
        from: TokenId,
        value: Uint128
    },
    Balance { owner: String, token_id: TokenId },
    BatchBalance {
        owner: String,
        token_ids: Vec<TokenId>,
    },
    ApprovedForAll {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    IsApprovedForAll { owner: String, operator: String },
    TokenInfo { token_id: TokenId },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SpotPriceResponse {
    pub spot_price: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TokenStateResponse {
    pub reserve: Uint128,
    pub supply: Uint128,
    pub funds: Uint128,
    pub funded: bool,
    pub reward: u64,
    pub locked: bool,
    pub created: u64,
    pub init_price: Decimal
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FundsFromNeuronResponse {
    pub funds: Vec<FundFromNeuron>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FundFromNeuron {
    pub token_id: TokenId,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FundsForNeuronResponse {
    pub funds: Vec<FundForNeuron>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FundForNeuron {
    pub address: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct NeuronVestingsResponse {
    pub vestings: Vec<Vesting>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Vesting {
    pub token_id: TokenId,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FundsResponse {
    pub funds: Vec<Fund>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Fund {
    pub token_id: TokenId,
    pub height: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SwapResponse {
    pub from: TokenId,
    pub to: TokenId,
    pub sell: Uint128,
    pub buy: Uint128
}
