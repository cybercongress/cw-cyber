use crate::state::{Entry};
// use cosmwasm_std::{Uint64};
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
    // TODO add change owner contract
    // TODO add change owner item
    NewEntry {
        neuron: String,
        network: String,
        protocol: String,
        endpoint: String,
        particle: Option<String>,
    },
    UpdateEntry {
        id: u64,
        neuron: Option<String>,
        network: Option<String>,
        protocol: Option<String>,
        endpoint: Option<String>,
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
        protocol: Option<String>,
        limit: Option<u32>,
    },
}

// Tokens
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub id: u64,
    pub neuron: String,
    pub protocol: String,
    pub endpoint: String,
    pub particle: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}
