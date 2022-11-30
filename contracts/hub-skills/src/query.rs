use cosmwasm_std::{Addr, Deps, Order, StdResult};
use cw_storage_plus::Bound;

use crate::msg::{EntryResponse, ListResponse};
use crate::state::entries;

const MAX_LIMIT: u32 = 50;
const DEFAULT_LIMIT: u32 = 30;

pub fn query_entry(deps: Deps, id: u64) -> StdResult<EntryResponse> {
    let entry = entries().load(deps.storage, id)?;
    Ok(EntryResponse {
        id,
        neuron: entry.neuron,
        protocol: entry.protocol,
        endpoint: entry.endpoint,
        particle: entry.particle
    })
}

pub fn query_list(deps: Deps, _start_after: Option<u64>, limit: Option<u32>, _protocol: Option<String>, owner: Option<Addr>) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let indexed_arr = entries().idx.owner;

    // TODO add start_after and protocol

    let entries: StdResult<Vec<_>> = indexed_arr
        .prefix(owner.clone().unwrap_or(Addr::unchecked("")).to_string())
        .range(
            deps.storage,
            None,
            None,
            Order::Ascending,
        )
        .take(limit)
        .collect();

    let result = ListResponse {
        entries: entries?.into_iter().map(|l| l.1).collect(),
    };

    Ok(result)
}