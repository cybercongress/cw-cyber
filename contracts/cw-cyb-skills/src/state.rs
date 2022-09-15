use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, UniqueIndex, IndexList, IndexedMap, Index, MultiIndex};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry {
    pub id: u64,
    pub neuron: String,
    pub protocol: String,
    pub endpoint: String,
    pub owner: Addr,
    pub particle: String,
}


pub const CONFIG: Item<Config> = Item::new("config");
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");



pub struct EntryIndexes<'a> {
    pub id: UniqueIndex<'a, u64, Entry>,
    pub owner: MultiIndex<'a, String, Entry, String>,
    pub protocol: MultiIndex<'a, String, Entry, String>,
    
}

impl<'a> IndexList<Entry> for EntryIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Entry>> + '_> {
        let v: Vec<&dyn Index<Entry>> = vec![&self.owner, &self.id];
        Box::new(v.into_iter())
    }
}

pub fn items<'a>() -> IndexedMap<'a, u64, Entry, EntryIndexes<'a>> {
    let indexes = EntryIndexes {
        owner: MultiIndex::new(
            |d| (d.owner.clone().to_string()),
            "list",
            "list_owner"
        ),
        protocol: MultiIndex::new(
            |d| (d.protocol.clone().to_string()),
            "list",
            "list_protocol"
        ),
        id: UniqueIndex::new(
            |d| (d.id.clone()),
            "list_id"
        ),

    };
    IndexedMap::new("list", indexes)
}