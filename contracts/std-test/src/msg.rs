use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cyber_std::{Link};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    MsgCyberlink {
        links: Vec<Link>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    RankValueById {
        cid_number: u64,
    },
    RankValueByCid {
        cid: String,
    },
    CidsCount {},
    LinksCount {},
}
