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
    ParticleRank {
        particle: String,
    },
    ParticlesAmount {},
    CyberlinksAmount {},
    Thought {
        program: String,
        name: String,
    },
    ThoughtStats {
        program: String,
        name: String,
    },
    LowestFee {},
    SourceRoutes {
        source: String,
    },
    SourceRoutedEnergy {
        source: String,
    },
    DestinationRoutedEnergy {
        destination: String,
    },
    Route {
        source: String,
        destination: String,
    },
    BandwidthPrice {},
    BandwidthLoad {},
    BandwidthTotal {},
    NeuronBandwidth {
        neuron: String,
    }
}

impl CustomQuery for CyberQueryWrapper {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParticleRankResponse {
    pub rank: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParticlesAmountResponse {
    pub particles_amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CyberlinksAmountResponse {
    pub cyberlinks_amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ThoughtResponse {
    pub program: String,
    pub trigger: Trigger,
    pub load: Load,
    pub name: String,
    pub particle: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ThoughtStatsResponse {
    pub program: String,
    pub name: String,
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
pub struct BandwidthPriceResponse {
    pub price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BandwidthLoadResponse {
    pub load: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BandwidthTotalResponse {
    pub total: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NeuronBandwidthResponse {
    pub neuron: String,
    pub remained_value: u64,
    pub last_updated_block: u64,
    pub max_value: u64,
}