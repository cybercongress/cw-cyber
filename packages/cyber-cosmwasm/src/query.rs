use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CustomQuery};
use crate::route::CyberRoute;

/// CyberQueryWrapper is an override of QueryRequest::Custom to access Cyber-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CyberQueryWrapper {
    pub route: CyberRoute,
    pub query_data: CyberQuery,
}

/// CyberQuery is defines avaliable query datas
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

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RankValueResponse {
    pub rank_value: u64,
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CidsCountResponse {
    pub cids_count: u64,
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LinksCountResponse {
    pub links_count: u64,
}
