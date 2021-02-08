use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


/// CyberRoute is enum type to represent cyber query route path
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CyberRoute {
    Rank,
    Graph,
}
