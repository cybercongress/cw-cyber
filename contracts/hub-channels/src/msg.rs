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
    UpdateOwner {
        new_owner: Option<String>,
    },
    CreateEntry {
        source_chain_id: String,
        destination_chain_id: String,
        source_channel_id: String,
        destination_channel_id: String,
        explorer_url: String,
        particle: Option<String>,
    },
    UpdateEntry {
        id: u64,
        source_chain_id: Option<String>,
        destination_chain_id: Option<String>,
        source_channel_id: Option<String>,
        destination_channel_id: Option<String>,
        explorer_url: Option<String>,
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
    pub source_chain_id: String,
    pub destination_chain_id: String,
    pub source_channel_id: String,
    pub destination_channel_id: String,
    pub explorer_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}
