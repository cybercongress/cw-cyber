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
        source_chain_id: String,
        destination_chain_id: String,
        source_channel_id: String,
        destination_channel_id: String,
        rpc: String,
        token: String,
    },
    UpdateEntry {
        id: u64,
        source_chain_id: Option<String>,
        destination_chain_id: Option<String>,
        source_channel_id: Option<String>,
        destination_channel_id: Option<String>,
        rpc: Option<String>,
        token: Option<String>,
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
    pub source_chain_id: String,
    pub destination_chain_id: String,
    pub source_channel_id: String,
    pub destination_channel_id: String,
    pub rpc: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}