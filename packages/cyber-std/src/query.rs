use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CustomQuery;
use crate::route::CyberRoute;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CyberQueryWrapper {
    pub route: CyberRoute,
    pub query_data: CyberQuery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CyberQuery {
    RankValueById {
        cid_number: u64,
    },
    RankValueByCid {
        cid: String,
    },
    CidsCount {},
    LinksCount {},
}

impl CustomQuery for CyberQueryWrapper {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RankValueResponse {
    pub rank_value: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CidsCountResponse {
    pub cids_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LinksCountResponse {
    pub links_count: u64,
}
