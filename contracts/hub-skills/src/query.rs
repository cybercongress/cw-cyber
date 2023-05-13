use cosmwasm_std::{Addr, Deps, Order, StdResult};
// use cw_storage_plus::Bound;

use crate::msg::{EntryResponse, ListResponse};
use crate::state::entries;

// const MAX_LIMIT: u32 = 50;
// const DEFAULT_LIMIT: u32 = 30;

pub fn query_entry(deps: Deps, id: u64) -> StdResult<EntryResponse> {
    let entry = entries().load(deps.storage, id)?;
    Ok(EntryResponse {
        id,
        neuron: entry.neuron,
        network: entry.network,
        protocol: entry.protocol,
        endpoint: entry.endpoint,
        owner: entry.owner.to_string(),
        particle: entry.particle
    })
}

pub fn query_list_by_owner(deps: Deps, _start_after: Option<u64>, _limit: Option<u32>, owner: Addr) -> StdResult<ListResponse> {
    // TODO
    // let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    // let start = start_after.map(Bound::exclusive);

    let indexed_arr = entries().idx.owner;

    let entries: StdResult<Vec<_>> = indexed_arr
        .prefix(owner.clone().to_string())
        .range(
            deps.storage,
            None,
            None,
            Order::Ascending,
        )
        // .take(limit)
        .collect();

    let result = ListResponse {
        entries: entries?.into_iter().map(|l| l.1).collect(),
    };

    Ok(result)
}

pub fn query_list_by_network(deps: Deps, _start_after: Option<u64>, _limit: Option<u32>, network: String) -> StdResult<ListResponse> {
    // TODO
    // let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    // let start = start_after.map(Bound::exclusive);

    let indexed_arr = entries().idx.network;

    let entries: StdResult<Vec<_>> = indexed_arr
        .prefix(network.clone().to_string())
        .range(
            deps.storage,
            None,
            None,
            Order::Ascending,
        )
        // .take(limit)
        .collect();

    let result = ListResponse {
        entries: entries?.into_iter().map(|l| l.1).collect(),
    };

    Ok(result)
}

pub fn query_list_by_protocol(deps: Deps, _start_after: Option<u64>, _limit: Option<u32>, protocol: String) -> StdResult<ListResponse> {
    // TODO
    // let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    // let start = start_after.map(Bound::exclusive);

    let indexed_arr = entries().idx.protocol;

    let entries: StdResult<Vec<_>> = indexed_arr
        .prefix(protocol.clone().to_string())
        .range(
            deps.storage,
            None,
            None,
            Order::Ascending,
        )
        // .take(limit)
        .collect();

    let result = ListResponse {
        entries: entries?.into_iter().map(|l| l.1).collect(),
    };

    Ok(result)
}