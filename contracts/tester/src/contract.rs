use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdResult, Storage, MessageInfo
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use cyber_cosmwasm::{
    CyberMsgWrapper, CyberQuerier, create_cyberlink_msg,
    RankValueResponse, CidsCountResponse, LinksCountResponse, Link,
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> StdResult<HandleResponse<CyberMsgWrapper>> {
    match msg {
        HandleMsg::MsgCyberlink {
            links,
        } =>  handle_msg_cyberlink(deps, env, info, links),
    }
}

pub fn handle_msg_cyberlink<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    env: Env,
    _info: MessageInfo,
    links: Vec<Link>
) -> StdResult<HandleResponse<CyberMsgWrapper>> {
    let contract = env.contract.address;
    let msg = create_cyberlink_msg(contract, links);

    let res = HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    };
    Ok(res)
}



pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::RankValueById { cid_number } => to_binary(&query_rank_value_by_id(deps, cid_number)?),
        QueryMsg::RankValueByCid { cid } => to_binary(&query_rank_value_by_cid(deps, cid)?),
        QueryMsg::CidsCount {} => to_binary(&query_cids_count(deps)?),
        QueryMsg::LinksCount {} => to_binary(&query_links_count(deps)?),
    }
}

pub fn query_rank_value_by_id<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    cid_number: u64,
) -> StdResult<RankValueResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: RankValueResponse = querier.query_rank_value_by_id(cid_number)?;

    Ok(res)
}

pub fn query_rank_value_by_cid<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    cid: String,
) -> StdResult<RankValueResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: RankValueResponse = querier.query_rank_value_by_cid(cid)?;

    Ok(res)
}

pub fn query_cids_count<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<CidsCountResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: CidsCountResponse = querier.query_cids_count()?;

    Ok(res)
}

pub fn query_links_count<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<LinksCountResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: LinksCountResponse = querier.query_links_count()?;

    Ok(res)
}
