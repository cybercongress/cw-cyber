use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Coin;
use cyber_std::{Link, Trigger, Load};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct  InstantiateMsg {
    pub creator: String,
    pub beats: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    Heartbeat {
        beats: u64,
    },
    Cyberlink {
        links: Vec<Link>,
    },
    Release {},
    CpuLoop {},
    StorageLoop {},
    MemoryLoop {},
    Panic {},
    TransferFunds {
        recipient: String,
        amount: Vec<Coin>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Cyberlink {
        links: Vec<Link>
    },
    Stake {
        validator: String,
        amount: Coin,
    },
    Unstake {
        validator: String,
        amount: Coin,
    },
    Investmint {
        amount: Coin,
        resource: String,
        length: u64,
    },
    CreateEnergyRoute {
        destination: String,
        alias: String,
    },
    EditEnergyRoute {
        destination: String,
        value: Coin,
    },
    EditEnergyRouteAlias {
        destination: String,
        alias: String,
    },
    DeleteEnergyRoute {
        destination: String,
    },
    CreateThought {
        trigger: Trigger,
        load: Load,
        name: String,
        particle: String,
    },
    ForgetThought {
        name: String,
    },
    ChangeThoughtCallData {
        name: String,
        call_data: String,
    },
    ChangeThoughtPeriod {
        name: String,
        period: u64,
    },
    ChangeThoughtBlock {
        name: String,
        block: u64,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ParticleRank {
        particle: String,
    },
    CyberlinksAmount {},
    ParticlesAmount {},
    Config {},
    Thought {
        program: String,
        name: String,
    },
    ThoughtStats {
        program: String,
        name: String,
    },
    DmnLowestFee {},
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