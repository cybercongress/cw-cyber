use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Coin;
use cyber_std::{Link, Trigger, Load, Route};

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
    AddJob {
        trigger: Trigger,
        load: Load,
        label: String,
        cid: String,
    },
    RemoveJob {
        label: String,
    },
    ChangeJobCallData {
        label: String,
        call_data: String,
    },
    ChangeJobPeriod {
        label: String,
        period: u64,
    },
    ChangeJobBlock {
        label: String,
        block: u64,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetRankValueByCid {
        cid: String,
    },
    GetCidsCount {},
    GetLinksCount {},
    Config {},
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