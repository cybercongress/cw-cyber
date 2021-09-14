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
        address: String,
        links: Vec<Link>,
    },
    Investmint {
        agent: String,
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
    AddJob {
        program: String,
        trigger: Trigger,
        load: Load,
        label: String,
        cid: String,
    },
    RemoveJob {
        program: String,
        label: String,
    },
    ChangeJobCallData {
        program: String,
        label: String,
        call_data: String,
    },
    ChangeJobPeriod {
        program: String,
        label: String,
        period: u64,
    },
    ChangeJobBlock {
        program: String,
        label: String,
        block: u64,
    }
}

pub fn create_cyberlink_msg(
    address: String,
    links: Vec<Link>,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Graph,
        msg_data: CyberMsg::Cyberlink {
            address,
            links,
        },
    }
    .into()
}

pub fn create_investmint_msg(
    agent: String,
    amount: Coin,
    resource: String,
    length: u64,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Resources,
        msg_data: CyberMsg::Investmint {
            agent,
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
        route: CyberRoute::Energy,
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
        route: CyberRoute::Energy,
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
        route: CyberRoute::Energy,
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
        route: CyberRoute::Energy,
        msg_data: CyberMsg::DeleteEnergyRoute {
            source,
            destination,
        },
    }
    .into()
}

pub fn create_add_job_msg(
    program: String,
    trigger: Trigger,
    load: Load,
    label: String,
    cid: String,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Cron,
        msg_data: CyberMsg::AddJob {
            program,
            trigger,
            load,
            label,
            cid,
        },
    }
    .into()
}

pub fn create_remove_job_msg(
    program: String,
    label: String,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Cron,
        msg_data: CyberMsg::RemoveJob {
            program,
            label,
        },
    }
    .into()
}

pub fn create_change_job_call_data_msg(
    program: String,
    label: String,
    call_data: String,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Cron,
        msg_data: CyberMsg::ChangeJobCallData {
            program,
            label,
            call_data,
        },
    }
    .into()
}

pub fn create_change_job_period_msg(
    program: String,
    label: String,
    period: u64,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Cron,
        msg_data: CyberMsg::ChangeJobPeriod {
            program,
            label,
            period,
        },
    }
    .into()
}

pub fn create_change_job_block_msg(
    program: String,
    label: String,
    block: u64,
) -> CosmosMsg<CyberMsgWrapper> {
    CyberMsgWrapper {
        route: CyberRoute::Cron,
        msg_data: CyberMsg::ChangeJobBlock {
            program,
            label,
            block,
        },
    }
    .into()
}