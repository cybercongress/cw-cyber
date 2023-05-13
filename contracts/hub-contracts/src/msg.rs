use crate::state::{Entry};
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
        address: String,
        query_cid: String,
        execute_cid: String,
        version: String,
        chain_id: String,
        particle: Option<String>,
    },
    UpdateEntry {
        id: u64,
        address: Option<String>,
        query_cid: Option<String>,
        execute_cid: Option<String>,
        version: Option<String>,
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
    pub address: String,
    pub query_cid: String,
    pub execute_cid: String,
    pub version: String,
    pub chain_id: String,
    pub particle: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}
