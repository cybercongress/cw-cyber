use cosmwasm_std::{Addr, Decimal, Uint128};
use cw1155::Expiration;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenState {
    pub reserve: Uint128,
    pub supply: Uint128,
    pub funds: Uint128,
    pub funded: bool,
    pub reward: u64,
    pub locked: bool,
    pub created: u64,
    pub init_price: Decimal
}

pub const MINTER: Item<Addr> = Item::new("minter");
pub const BALANCES: Map<(&Addr, &str), Uint128> = Map::new("balances");
pub const APPROVES: Map<(&Addr, &Addr), Expiration> = Map::new("approves");
// TODO reuse TOKENS
pub const TOKENS: Map<&str, String> = Map::new("tokens");
pub const TOKENS_STATES: Map<&str, TokenState> = Map::new("tokens_states");

pub const FUNDS_BY_BLOCKS: Map<u64, String> = Map::new("funds_by_blocks");
pub const FUNDS_FROM_NEURONS: Map<(&Addr, &str), Uint128> = Map::new("funds_from");
pub const FUNDS_FOR_NEURONS: Map<(&str, &Addr), Uint128> = Map::new("funds_for");
pub const VESTINGS: Map<(&Addr, &str), Uint128> = Map::new("vestings");
