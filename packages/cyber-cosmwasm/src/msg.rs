use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CosmosMsg, HumanAddr};
use crate::route::CyberRoute;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Link {
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// CyberMsgWrapper is an override of CosmosMsg::Custom to show this works and can be extended in the contract
pub struct CyberMsgWrapper {
    pub route: CyberRoute,
    pub msg_data: CyberMsg,
}

// this is a helper to be able to return these as CosmosMsg easier
impl Into<CosmosMsg<CyberMsgWrapper>> for CyberMsgWrapper {
    fn into(self) -> CosmosMsg<CyberMsgWrapper> {
        CosmosMsg::Custom(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CyberMsg {
    Cyberlink {
        address: HumanAddr,
        links: Vec<Link>,
    }
}

pub fn create_cyberlink_msg(
    address: HumanAddr,
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
