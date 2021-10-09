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
    create_creat_thought_msg, create_forget_thought_msg, create_change_thought_call_data_msg,
    create_change_thought_period_msg, create_change_thought_block_msg,
    ParticleRankResponse, ParticlesAmountResponse, CyberlinksAmountResponse,
    ThoughtResponse, ThoughtStatsResponse, LowestFeeResponse,
    RouteResponse, RoutesResponse, RoutedEnergyResponse,
    BandwidthPriceResponse, BandwidthLoadResponse, BandwidthTotalResponse, NeuronBandwidthResponse,
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
        ExecuteMsg::CreateThought {
            trigger,
            load,
            name,
            particle,
        } => create_thought(deps, env, info, trigger, load, name, particle),
        ExecuteMsg::ForgetThought {
            name,
        } => forget_thought(deps, env, info, name),
        ExecuteMsg::ChangeThoughtCallData {
            name,
            call_data,
        } => change_thought_call_data(deps, env, info, name, call_data),
        ExecuteMsg::ChangeThoughtPeriod {
            name,
            period,
        } => change_thought_period(deps, env, info, name, period),
        ExecuteMsg::ChangeThoughtBlock {
            name,
            block,
        } => change_thought_block(deps, env, info, name, block),
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

pub fn create_thought(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    trigger: Trigger,
    load: Load,
    name: String,
    particle: String,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_creat_thought_msg(
        contract.into(),
        trigger.into(),
        load.into(),
        name.into(),
        particle.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn forget_thought(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    name: String,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_forget_thought_msg(
        contract.into(),
        name.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn change_thought_call_data(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    name: String,
    call_data: String,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_change_thought_call_data_msg(
        contract.into(),
        name.into(),
        call_data.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn change_thought_period(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    name: String,
    period: u64,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_change_thought_period_msg(
        contract.into(),
        name.into(),
        period.into(),
    );

    let res = Response::new()
        .add_message(msg);
    Ok(res)
}

pub fn change_thought_block(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    name: String,
    block: u64,
) -> Result<Response<CyberMsgWrapper>, ContractError> {
    let contract = env.contract.address;
    let msg = create_change_thought_block_msg(
        contract.into(),
        name.into(),
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
        QueryMsg::ParticleRank {
            particle
        } => to_binary(&query_particle_rank(deps, particle)?),
        QueryMsg::ParticlesAmount {} => to_binary(&query_particles_amount(deps)?),
        QueryMsg::CyberlinksAmount {} => to_binary(&query_cyberlinks_amount(deps)?),
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
        QueryMsg::Thought {
            program,
            name,
        } => to_binary(&query_thought(deps, program, name)?),
        QueryMsg::ThoughtStats {
            program,
            name,
        } => to_binary(&query_thought_stats(deps, program, name)?),
        QueryMsg::DmnLowestFee {} => to_binary(&query_lowest_fee(deps)?),
        QueryMsg::SourceRoutes {
            source,
        } => to_binary(&query_source_routes(deps, source)?),
        QueryMsg::SourceRoutedEnergy {
            source,
        } => to_binary(&query_source_routed_energy(deps, source)?),
        QueryMsg::DestinationRoutedEnergy {
            destination,
        } => to_binary(&query_destination_routed_energy(deps,destination)?),
        QueryMsg::Route {
            source,
            destination,
        } => to_binary(&query_route(deps, source, destination)?),
        QueryMsg::BandwidthPrice {} => to_binary(&query_price(deps)?),
        QueryMsg::BandwidthLoad {} => to_binary(&query_load(deps)?),
        QueryMsg::BandwidthTotal {} => to_binary(&query_desirable_bandwidth(deps)?),
        QueryMsg::NeuronBandwidth {
            neuron,
        } => to_binary(&query_neuron_bandwidth(deps, neuron)?),
    }
}

pub fn query_particle_rank(
    deps: Deps,
    particle: String,
) -> StdResult<ParticleRankResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: ParticleRankResponse = querier.query_particle_rank(particle)?;

    Ok(res)
}

pub fn query_particles_amount(
    deps: Deps,
) -> StdResult<ParticlesAmountResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: ParticlesAmountResponse = querier.query_particles_amount()?;

    Ok(res)
}

pub fn query_cyberlinks_amount(
    deps: Deps,
) -> StdResult<CyberlinksAmountResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: CyberlinksAmountResponse = querier.query_cyberlinks_amount()?;

    Ok(res)
}

pub fn query_thought(
    deps: Deps,
    program: String,
    name: String,
) -> StdResult<ThoughtResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: ThoughtResponse = querier.query_thought(program, name)?;

    Ok(res)
}

pub fn query_thought_stats(
    deps: Deps,
    program: String,
    name: String,
) -> StdResult<ThoughtStatsResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: ThoughtStatsResponse = querier.query_thought_stats(program, name)?;

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
) -> StdResult<BandwidthPriceResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: BandwidthPriceResponse = querier.query_bandwidth_price()?;

    Ok(res)
}

pub fn query_load(
    deps: Deps,
) -> StdResult<BandwidthLoadResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: BandwidthLoadResponse = querier.query_bandwidth_load()?;

    Ok(res)
}

pub fn query_desirable_bandwidth(
    deps: Deps,
) -> StdResult<BandwidthTotalResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: BandwidthTotalResponse = querier.query_bandwidth_total()?;

    Ok(res)
}

pub fn query_neuron_bandwidth(
    deps: Deps,
    address: String
) -> StdResult<NeuronBandwidthResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: NeuronBandwidthResponse = querier.query_neuron_bandwidth(address)?;

    Ok(res)
}