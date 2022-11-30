use crate::state::{Entry};
use cosmwasm_std::{Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    UpdateOwner {
        new_owner: Option<String>,
    },
    CreateEntry {
        ticker: String,
        chain_id: String,
        denom: String,
        channel: Uint64,
        logo: String,
        particle: Option<String>,
    },
    UpdateEntry {
        id: u64,
        ticker: Option<String>,
        chain_id: Option<String>,
        denom: Option<String>,
        channel: Option<Uint64>,
        logo: Option<String>,
        particle: Option<String>,
    },
    DeleteEntry {
        id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetEntries {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    GetEntry { id: u64 }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub id: u64,
    pub ticker: String,
    pub chain_id: String,
    pub denom: String,
    pub channel: Uint64,
    pub logo: String,
    pub particle: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}
