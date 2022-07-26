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
    NewEntry {
        ticker: String,
        name: String,
        denom: Uint64,
        logo: String,
        // active: bool,
        // order: Option<Uint64>
    },
    UpdateEntry {
        id: u64,
        ticker: Option<String>,
        name: Option<String>,
        denom: Option<Uint64>,
        logo: Option<String>,
        // active: Option<bool>,
        // order: Option<Uint64>
    },
    DeleteEntry {
        id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetTokens {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

// Tokens
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub id: u64,
    pub ticker: String,
    pub name: String,
    pub denom: Uint64,
    pub logo: String,
    // pub active: Option<bool>,
    // pub order: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}
