use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CustomQuery, Coin};
use crate::route::CyberRoute;
use crate::msg::{Trigger, Load, Route};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CyberQueryWrapper {
    pub route: CyberRoute,
    pub query_data: CyberQuery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CyberQuery {
    // RankValueById {
    //     cid_number: u64,
    // },
    GetRankValueByCid {
        cid: String,
    },
    GetCidsCount {},
    GetLinksCount {},
    GetJob {
        creator: String,
        contract: String,
        label: String,
    },
    GetJobStats {
        creator: String,
        contract: String,
        label: String,
    },
    GetLowestFee {},
    GetSourceRoutes {
        source: String,
    },
    GetSourceRoutedEnergy {
        source: String,
    },
    GetDestinationRoutedEnergy {
        destination: String,
    },
    GetRoute {
        source: String,
        destination: String,
    },
    GetPrice {},
    GetLoad {},
    GetDesirableBandwidth {},
    GetAccountBandwidth {
        address: String,
    }
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct JobResponse {
    pub creator: String,
    pub contract: String,
    pub trigger: Trigger,
    pub load: Load,
    pub label: String,
    pub cid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct JobStatsResponse {
    pub creator: String,
    pub contract: String,
    pub label: String,
    pub calls: u64,
    pub fees: u64,
    pub gas: u64,
    pub last_block: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LowestFeeResponse {
    pub fee: Coin,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoutesResponse {
    pub routes: Vec<Route>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoutedEnergyResponse {
    pub value: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RouteResponse {
    pub route: Route,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LoadResponse {
    pub load: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DesirableBandwidthResponse {
    pub desirable_bandwidth: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AccountBandwidthResponse {
    pub address: String,
    pub remained_value: u64,
    pub last_updated_block: u64,
    pub max_value: u64,
}