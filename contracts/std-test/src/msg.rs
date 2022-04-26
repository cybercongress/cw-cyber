use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Decimal};
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
        name: String,
    },
    EditEnergyRoute {
        destination: String,
        value: Coin,
    },
    EditEnergyRouteName {
        destination: String,
        name: String,
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
    ChangeThoughtInput {
        name: String,
        input: String,
    },
    ChangeThoughtPeriod {
        name: String,
        period: u64,
    },
    ChangeThoughtBlock {
        name: String,
        block: u64,
    },
    CreatePool {
        pool_type_id: u32,
        deposit_coins: Vec<Coin>,
    },
    DepositWithinBatch {
        pool_id: u64,
        deposit_coins: Vec<Coin>,
    },
    WithdrawWithinBatch {
        pool_id: u64,
        pool_coin: Coin,
    },
    SwapWithinBatch {
        pool_id: u64,
        swap_type_id: u32,
        offer_coin: Coin,
        demand_coin_denom: String,
        offer_coin_fee: Coin,
        order_price: Decimal,
    },
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
    ThoughtLowestFee {},
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
    },
    PoolParams {
        pool_id: u64,
    },
    PoolLiquidity {
        pool_id: u64,
    },
    PoolSupply {
        pool_id: u64,
    },
    PoolPrice {
        pool_id: u64,
    },
    PoolAddress {
        pool_id: u64,
    },
}