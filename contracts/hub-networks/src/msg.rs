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
    NewEntry {
        name: String,
        chain_id: String,
        prefix: String,
        genesis_hash: String,
        protocol: String,
        unbonding_period: String,
        logo: String,
        particle: Option<String>,
    },
    UpdateEntry {
        id: u64,
        name: Option<String>,
        chain_id: Option<String>,
        prefix: Option<String>,
        genesis_hash: Option<String>,
        protocol: Option<String>,
        unbonding_period: Option<String>,
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
    GetItems {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub id: u64,
    pub name: String,
    pub chain_id: String,
    pub prefix: String,
    pub genesis_hash: String,
    pub protocol: String,
    pub unbonding_period: String,
    pub logo: String,
    pub particle: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}
