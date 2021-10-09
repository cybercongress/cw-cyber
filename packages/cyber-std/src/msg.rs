use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CosmosMsg, Coin};
use crate::route::CyberRoute;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Link {
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Trigger {
    pub period: u64,
    pub block: u64,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Load {
    pub call_data: String,
    pub gas_price: Coin,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Route {
    pub source: String,
    pub destination: String,
    pub alias: String,
    pub value: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// CyberMsgWrapper is an override of CosmosMsg::Custom to show this works and can be extended in the contract
pub struct CyberMsgWrapper {
    pub route: CyberRoute,
    pub msg_data: CyberMsg,
}

impl Into<CosmosMsg<CyberMsgWrapper>> for CyberMsgWrapper {
    fn into(self) -> CosmosMsg<CyberMsgWrapper> {
        CosmosMsg::Custom(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CyberMsg {
    Cyberlink {
        neuron: String,
        links: Vec<Link>,
    },
    Investmint {
        neuron: String,
        amount: Coin,
        resource: String,
        length: u64,
    },
    CreateEnergyRoute {
        source: String,
        destination: String,
        alias: String,
    },
    EditEnergyRoute {
        source: String,
        destination: String,
        value: Coin,
    },
    EditEnergyRouteAlias {
        source: String,
        destination: String,
        alias: String,
    },
    DeleteEnergyRoute {
        source: String,
        destination: String,
    },
    CreateThought {
        program: String,
        trigger: Trigger,
        load: Load,
        name: String,
        particle: String,
    },
    ForgetThought {
        program: String,
        name: String,
    },
    ChangeThoughtCallData {
        program: String,
        name: String,
        call_data: String,
    },
    ChangeThoughtPeriod {
        program: String,
        name: String,
        period: u64,
    },
    ChangeThoughtBlock {
        program: String,
        name: String,
        block: u64,
    }
}

pub fn create_cyberlink_msg(
    neuron: String,
    links: Vec<Link>,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Graph,
        msg_data: CyberMsg::Cyberlink {
            neuron,
            links,
        },
    }
    .into()
}

pub fn create_investmint_msg(
    neuron: String,
    amount: Coin,
    resource: String,
    length: u64,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Resources,
        msg_data: CyberMsg::Investmint {
            neuron,
            amount,
            resource,
            length,
        },
    }
    .into()
}

pub fn create_create_energy_route_msg(
    source: String,
    destination: String,
    alias: String,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Grid,
        msg_data: CyberMsg::CreateEnergyRoute {
            source,
            destination,
            alias,
        },
    }
    .into()
}

pub fn create_edit_energy_route_msg(
    source: String,
    destination: String,
    value: Coin,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Grid,
        msg_data: CyberMsg::EditEnergyRoute {
            source,
            destination,
            value,
        },
    }
    .into()
}

pub fn create_edit_energy_route_alias_msg(
    source: String,
    destination: String,
    alias: String,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Grid,
        msg_data: CyberMsg::EditEnergyRouteAlias {
            source,
            destination,
            alias,
        },
    }
    .into()
}

pub fn create_delete_energy_route_msg(
    source: String,
    destination: String,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Grid,
        msg_data: CyberMsg::DeleteEnergyRoute {
            source,
            destination,
        },
    }
    .into()
}

pub fn create_creat_thought_msg(
    program: String,
    trigger: Trigger,
    load: Load,
    name: String,
    particle: String,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Dmn,
        msg_data: CyberMsg::CreateThought {
            program,
            trigger,
            load,
            name,
            particle,
        },
    }
    .into()
}

pub fn create_forget_thought_msg(
    program: String,
    name: String,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Dmn,
        msg_data: CyberMsg::ForgetThought {
            program,
            name,
        },
    }
    .into()
}

pub fn create_change_thought_call_data_msg(
    program: String,
    name: String,
    call_data: String,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Dmn,
        msg_data: CyberMsg::ChangeThoughtCallData {
            program,
            name,
            call_data,
        },
    }
    .into()
}

pub fn create_change_thought_period_msg(
    program: String,
    name: String,
    period: u64,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Dmn,
        msg_data: CyberMsg::ChangeThoughtPeriod {
            program,
            name,
            period,
        },
    }
    .into()
}

pub fn create_change_thought_block_msg(
    program: String,
    name: String,
    block: u64,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Dmn,
        msg_data: CyberMsg::ChangeThoughtBlock {
            program,
            name,
            block,
        },
    }
    .into()
}