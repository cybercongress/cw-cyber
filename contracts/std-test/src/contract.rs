use cosmwasm_std::{to_binary, Binary, Env, Response, StdResult, MessageInfo, DepsMut, Deps};
use crate::error::Never;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use cyber_std::{
    CyberMsgWrapper, CyberQuerier, create_cyberlink_msg,
    RankValueResponse, CidsCountResponse, LinksCountResponse, Link,
};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}


pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<Response<CyberMsgWrapper>, Never> {
    match msg {
        HandleMsg::MsgCyberlink {
            links,
        } =>  handle_msg_cyberlink(deps, env, info, links),
    }
}

pub fn handle_msg_cyberlink(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    links: Vec<Link>
) -> Result<Response<CyberMsgWrapper>, Never> {
    let contract = env.contract.address;
    let msg = create_cyberlink_msg(contract, links);

    let res = Response {
        submessages: vec![],
        messages: vec![msg],
        attributes: vec![],
        data: None,
    };
    Ok(res)
}



pub fn query(
    deps: Deps,
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

pub fn query_rank_value_by_id(
    deps: Deps,
    cid_number: u64,
) -> StdResult<RankValueResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: RankValueResponse = querier.query_rank_value_by_id(cid_number)?;

    Ok(res)
}

pub fn query_rank_value_by_cid(
    deps: Deps,
    cid: String,
) -> StdResult<RankValueResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: RankValueResponse = querier.query_rank_value_by_cid(cid)?;

    Ok(res)
}

pub fn query_cids_count(
    deps: Deps,
) -> StdResult<CidsCountResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: CidsCountResponse = querier.query_cids_count()?;

    Ok(res)
}

pub fn query_links_count(
    deps: Deps,
) -> StdResult<LinksCountResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: LinksCountResponse = querier.query_links_count()?;

    Ok(res)
}