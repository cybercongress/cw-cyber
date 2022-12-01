use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;

use crate::msg::{EntryResponse, ListResponse};
use crate::state::LIST;

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 20;

pub fn query_entry(deps: Deps, id: u64) -> StdResult<EntryResponse> {
    let entry = LIST.load(deps.storage, id)?;

    Ok(EntryResponse {
        id: entry.id,
        address: entry.address,
        query_cid: entry.query_cid,
        execute_cid: entry.execute_cid,
        version: entry.version,
        chain_id: entry.chain_id,
        particle: entry.particle
    })
}

pub fn query_list(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let entries: StdResult<Vec<_>> = LIST
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect();

    let result = ListResponse {
        entries: entries?.into_iter().map(|l| l.1).collect(),
    };

    Ok(result)
}