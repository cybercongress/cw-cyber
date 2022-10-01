use crate::state::{Entry};
use cosmwasm_std::{Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{
   Addr,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    NewEntry {
        address: String,
        chain_id: String,
        particle: Option<String>,
    },
    UpdateEntry {
        id: u64,
        address: Option<String>,
        chain_id: Option<String>,
        particle: Option<String>,
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
        owner: Option<Addr>,
        id: Option<u64>,
        limit: Option<u32>,
    },
}

// Tokens
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub id: u64,
    pub address: String,
    pub chain_id: String,
    // pub owner: String,
    pub particle: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}
