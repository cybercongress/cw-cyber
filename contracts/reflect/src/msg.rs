use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, CosmosMsg, QueryRequest, SubMsg};
use cyber_std::{CyberMsgWrapper, CyberQueryWrapper};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReflectMsg { msgs: Vec<CosmosMsg<CyberMsgWrapper>> },
    ReflectSubMsg { msgs: Vec<SubMsg<CyberMsgWrapper>> },
    ChangeOwner { owner: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Owner {},
    /// Queries the blockchain and returns the result untouched
    Chain {
        request: QueryRequest<CyberQueryWrapper>,
    },
    /// Queries another contract and returns the data
    Raw {
        contract: String,
        key: Binary,
    },
    /// If there was a previous ReflectSubMsg with this ID, returns cosmwasm_std::Reply
    SubMsgResult {
        id: u64,
    },
}

// We define a custom struct for each query response

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainResponse {
    pub data: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RawResponse {
    /// The returned value of the raw query. Empty data can be the
    /// result of a non-existent key or an empty value. We cannot
    /// differentiate those two cases in cross contract queries.
    pub data: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// The response data for all `CyberQuery`s
pub struct CyberQueryResponse {
    pub msg: String,
}
