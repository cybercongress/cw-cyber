use crate::state::{Entry};
// use cosmwasm_std::{Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    NewEntry {
        name: String,
        chain_id: String,
        genesis_hash: String,
        unbonding_period: String,
        logo: String,
        github: String,
    },
    UpdateEntry {
        id: u64,
        name: Option<String>,
        chain_id: Option<String>,
        genesis_hash: Option<String>,
        unbonding_period: Option<String>,
        logo: Option<String>,
        github: Option<String>,
    },
    DeleteEntry {
        id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetItems {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub id: u64,
    pub name: String,
    pub chain_id: String,
    pub genesis_hash: String,
    pub unbonding_period: String,
    pub logo: String,
    pub github: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}
