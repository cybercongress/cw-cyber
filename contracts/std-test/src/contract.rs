use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint64, Coin, StakingMsg, attr, BankMsg, from_slice, coin
};
use schemars::JsonSchema;
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, SudoMsg, QueryMsg};
use crate::state::{State, config,config_read};
use cyber_std::{
    CyberMsgWrapper, CyberQuerier,
    Link, Trigger, Load, Route,
    create_cyberlink_msg, create_investmint_msg,
    create_create_energy_route_msg, create_edit_energy_route_msg,
    create_edit_energy_route_alias_msg, create_delete_energy_route_msg,
    create_add_job_msg, create_remove_job_msg, create_change_job_call_data_msg,
    create_change_job_period_msg, create_change_job_block_msg,
    RankValueResponse, CidsCountResponse, LinksCountResponse,
    JobResponse, JobStatsResponse, LowestFeeResponse,
    RouteResponse, RoutesResponse, RoutedEnergyResponse,
    PriceResponse, LoadResponse, DesirableBandwidthResponse, AccountBandwidthResponse,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let state = State {
        creator: info.sender.into(),
        beats: msg.beats,
    };

    config(deps.storage).save(&state)?;

    Ok(Response::default())
}
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::Cyberlink { links } => cyberlink(deps, env, links),
        ExecuteMsg::Stake {
            validator,
            amount
        } => stake(deps, env, info, validator, amount),
        ExecuteMsg::Unstake {
            validator,
            amount
        } => unstake(deps, env, info, validator, amount),
        ExecuteMsg::Investmint {
            amount,
            resource,
            length,
        } => investmint(deps, env, info, amount, resource, length),
        ExecuteMsg::CreateEnergyRoute {
            destination,
            alias,
        } => create_energy_route(deps, env, info, destination, alias),
        ExecuteMsg::EditEnergyRoute {
            destination,
            value,
        } => edit_energy_route(deps, env, info, destination, value),
        ExecuteMsg::EditEnergyRouteAlias {
            destination,
            alias,
        } => edit_energy_route_alias(deps, env, info, destination, alias),
        ExecuteMsg::DeleteEnergyRoute {
            destination,
        } => delete_energy_route(deps, env, info, destination),
        ExecuteMsg::AddJob {
            trigger,
            load,
            label,
            cid,
        } => add_job(deps, env, info, trigger, load, label, cid),
        ExecuteMsg::RemoveJob {
            label,
        } => remove_job(deps, env, info, label),
        ExecuteMsg::ChangeJobCallData {
            label,
            call_data,
        } => change_job_call_data(deps, env, info, label, call_data),
        ExecuteMsg::ChangeJobPeriod {
            label,
            period,
        } => change_job_period(deps, env, info, label, period),
        ExecuteMsg::ChangeJobBlock {
            label,
            block,
        } => change_job_block(deps, env, info, label, block),
    }
}

pub fn cyberlink(
    _deps: DepsMut,
    env: Env,
    links: Vec<Link>
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_cyberlink_msg(contract.into(), links);

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn stake(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator: String,
    amount: Coin,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let amount = coin(u128::from(amount.amount), amount.denom);
    let res = Response::new()
        .add_message(StakingMsg::Delegate {
            validator: validator.into(),
            amount: amount.clone(),
        });
    Ok(res)
}

pub fn unstake(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator: String,
    amount: Coin,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let amount = coin(u128::from(amount.amount), amount.denom);
    let res = Response::new()
        .add_message(StakingMsg::Undelegate {
            validator: validator.into(),
            amount: amount.clone(),
        });
    Ok(res)
}

pub fn investmint(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    amount: Coin,
    resource: String,
    length: u64,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let amount = coin(u128::from(amount.amount), amount.denom);
    let agent = env.contract.address;
    let msg = create_investmint_msg(
        agent.into(),
        amount.clone(),
        resource.into(),
        length.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn create_energy_route(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination: String,
    alias: String,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_create_energy_route_msg(
        contract.into(),
        destination.into(),
        alias.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn edit_energy_route(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination: String,
    value: Coin
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let value = coin(u128::from(value.amount), value.denom);
    let msg = create_edit_energy_route_msg(
        contract.into(),
        destination.into(),
        value.clone(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn edit_energy_route_alias(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination: String,
    alias: String
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_edit_energy_route_alias_msg(
        contract.into(),
        destination.into(),
        alias.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn delete_energy_route(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination: String,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_delete_energy_route_msg(
        contract.into(),
        destination.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn add_job(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    trigger: Trigger,
    load: Load,
    label: String,
    cid: String,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_add_job_msg(
        contract.into(),
        trigger.into(),
        load.into(),
        label.into(),
        cid.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn remove_job(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    label: String,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_remove_job_msg(
        contract.into(),
        label.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn change_job_call_data(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    label: String,
    call_data: String,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_change_job_call_data_msg(
        contract.into(),
        label.into(),
        call_data.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn change_job_period(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    label: String,
    period: u64,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_change_job_period_msg(
        contract.into(),
        label.into(),
        period.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn change_job_block(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    label: String,
    block: u64,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_change_job_block_msg(
        contract.into(),
        label.into(),
        block.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

#[entry_point]
pub fn sudo(
    deps: DepsMut,
    env: Env,
    msg: SudoMsg
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    match msg {
        SudoMsg::Heartbeat { beats } => do_beat(deps, env, beats),
        SudoMsg::Cyberlink { links } => cyberlink(deps, env, links),
        SudoMsg::Release {} => do_release(deps, env),
        SudoMsg::CpuLoop {} => do_cpu_loop(),
        SudoMsg::StorageLoop {} => do_storage_loop(deps),
        SudoMsg::MemoryLoop {} => do_memory_loop(),
        SudoMsg::Panic {} => do_panic(),
        SudoMsg::TransferFunds { recipient, amount } => {
            let response = Response::new()
                .add_message(BankMsg::Send {
                    to_address: recipient,
                    amount,
                });
            Ok(response)
        }
    }
}

fn do_beat(deps: DepsMut, _env: Env, beats: u64) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let mut state = config(deps.storage).load()?;

    state.beats = state.beats + beats;

    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

fn do_release(deps: DepsMut, env: Env) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let state = config(deps.storage).load()?;

    let to_addr = state.creator;
    let balance = deps.querier.query_all_balances(env.contract.address)?;

    let resp = Response::new()
        .add_attribute("action", "release")
        .add_attribute("destination", to_addr.clone())
        .add_message(BankMsg::Send {
            to_address: to_addr.into(),
            amount: balance,
        })
        .set_data(&[0xF0, 0x0B, 0xAA]);
    Ok(resp)
}

fn do_cpu_loop() -> Result<Response<CyberMsgWrapper>, ContractError> {
    let mut counter = 0u64;
    loop {
        counter += 1;
        if counter >= 9_000_000_000 {
            counter = 0;
        }
    }
}

fn do_storage_loop(deps: DepsMut) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let mut test_case = 0u64;
    loop {
        deps.storage
            .set(b"test.key", test_case.to_string().as_bytes());
        test_case += 1;
    }
}

fn do_memory_loop() -> Result<Response<CyberMsgWrapper>, ContractError> {
    let mut data = vec![1usize];
    loop {
        // add one element
        data.push((*data.last().expect("must not be empty")) + 1);
    }
}

fn do_panic() -> Result<Response<CyberMsgWrapper>, ContractError> {
    panic!("This page intentionally faulted");
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRankValueByCid { cid } => to_binary(&query_rank_value_by_cid(deps, cid)?),
        QueryMsg::GetCidsCount {} => to_binary(&query_cids_count(deps)?),
        QueryMsg::GetLinksCount {} => to_binary(&query_links_count(deps)?),
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
        QueryMsg::GetJob {
            program,
            label,
        } => to_binary(&query_job(deps, program, label)?),
        QueryMsg::GetJobStats {
            program,
            label,
        } => to_binary(&query_job_stats(deps, program, label)?),
        QueryMsg::GetLowestFee {} => to_binary(&query_lowest_fee(deps)?),
        QueryMsg::GetSourceRoutes {
            source,
        } => to_binary(&query_source_routes(deps, source)?),
        QueryMsg::GetSourceRoutedEnergy {
            source,
        } => to_binary(&query_source_routed_energy(deps, source)?),
        QueryMsg::GetDestinationRoutedEnergy {
            destination,
        } => to_binary(&query_destination_routed_energy(deps,destination)?),
        QueryMsg::GetRoute {
            source,
            destination,
        } => to_binary(&query_route(deps, source, destination)?),
        QueryMsg::GetPrice {} => to_binary(&query_price(deps)?),
        QueryMsg::GetLoad {} => to_binary(&query_load(deps)?),
        QueryMsg::GetDesirableBandwidth {} => to_binary(&query_desirable_bandwidth(deps)?),
        QueryMsg::GetAccountBandwidth {
            address,
        } => to_binary(&query_account_bandwidth(deps, address)?),
    }
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

pub fn query_job(
    deps: Deps,
    program: String,
    label: String,
) -> StdResult<JobResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: JobResponse = querier.query_job(program, label)?;

    Ok(res)
}

pub fn query_job_stats(
    deps: Deps,
    program: String,
    label: String,
) -> StdResult<JobStatsResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: JobStatsResponse = querier.query_job_stats(program, label)?;

    Ok(res)
}

pub fn query_lowest_fee(
    deps: Deps,
) -> StdResult<LowestFeeResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: LowestFeeResponse = querier.query_lowest_fee()?;

    Ok(res)
}

pub fn query_source_routes(
    deps: Deps,
    source: String,
) -> StdResult<RoutesResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: RoutesResponse = querier.query_source_routes(source)?;

    Ok(res)
}

pub fn query_source_routed_energy(
    deps: Deps,
    source: String,
) -> StdResult<RoutedEnergyResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: RoutedEnergyResponse = querier.query_source_routed_energy(source)?;

    Ok(res)
}

pub fn query_destination_routed_energy(
    deps: Deps,
    destination: String,
) -> StdResult<RoutedEnergyResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: RoutedEnergyResponse = querier.query_destination_routed_energy(destination)?;

    Ok(res)
}

pub fn query_route(
    deps: Deps,
    source: String,
    destination: String,
) -> StdResult<RouteResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: RouteResponse = querier.query_route(source, destination)?;

    Ok(res)
}

pub fn query_price(
    deps: Deps,
) -> StdResult<PriceResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: PriceResponse = querier.query_price()?;

    Ok(res)
}

pub fn query_load(
    deps: Deps,
) -> StdResult<LoadResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: LoadResponse = querier.query_load()?;

    Ok(res)
}

pub fn query_desirable_bandwidth(
    deps: Deps,
) -> StdResult<DesirableBandwidthResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: DesirableBandwidthResponse = querier.query_desirable_bandwidth()?;

    Ok(res)
}

pub fn query_account_bandwidth(
    deps: Deps,
    address: String
) -> StdResult<AccountBandwidthResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: AccountBandwidthResponse = querier.query_account_bandwidth(address)?;

    Ok(res)
}