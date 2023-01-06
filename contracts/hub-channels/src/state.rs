use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use cosmwasm_std::{Uint64};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry {
    pub id: u64,
    pub active: bool,
    pub source_chain_id: String,
    pub destination_chain_id: String,
    pub source_channel_id: String,
    pub destination_channel_id: String,
    pub explorer_url: String,
    pub particle: String,
    pub(crate) active: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
pub const LIST: Map<u64, Entry> = Map::new("list");
