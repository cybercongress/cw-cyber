use std::fmt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CosmosMsg, Empty};
use cw3::{ProposalResponse, Vote};
use cw4::MemberChangedHookMsg;
use cw_utils::{Duration, Expiration, Threshold};

use crate::state::Executor;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    // this is the group contract that contains the member list
    pub group_addr: String,
    pub threshold: Threshold,
    pub max_voting_period: Duration,
    // who is able to execute passed proposals
    // None means that anyone can execute
    pub executor: Option<Executor>,
}

// TODO: add some T variants? Maybe good enough as fixed Empty for now
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T = Empty>
    where
        T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    Propose {
        title: String,
        description: String,
        msgs: Vec<CosmosMsg<T>>,
        // note: we ignore API-spec'd earliest if passed, always opens immediately
        latest: Option<Expiration>,
    },
    Vote {
        proposal_id: u64,
        vote: Vote,
    },
    Execute {
        proposal_id: u64,
    },
    Close {
        proposal_id: u64,
    },
    /// Handles update hook messages from the group contract
    MemberChangedHook(MemberChangedHookMsg),
}

// We can also add this as a cw3 extension
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return ThresholdResponse
    Threshold {},
    /// Returns ProposalResponse
    Proposal { proposal_id: u64 },
    /// Returns ProposalListResponse
    ListProposals {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    /// Returns ProposalListResponse
    ReverseProposals {
        start_before: Option<u64>,
        limit: Option<u32>,
    },
    /// Returns VoteResponse
    Vote { proposal_id: u64, voter: String },
    /// Returns VoteListResponse
    ListVotes {
        proposal_id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns VoterInfo
    Voter { address: String },
    /// Returns VoterListResponse
    ListVoters {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg<T = Empty>
    where
        T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    Execute {
        msgs: Vec<CosmosMsg<T>>,
    }
}

// TODO bring PR with ProposalListResponse to cw-plus
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ProposalListResponse<T = Empty>
    where
        T: Clone + fmt::Debug + PartialEq + JsonSchema
{
    pub proposals: Vec<ProposalResponse<T>>,
}