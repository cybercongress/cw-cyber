use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{config, config_read, State};
use cosmwasm_std::{coin, entry_point, to_binary, BankMsg, Binary, Coin, Env, MessageInfo, StakingMsg, StdError, StdResult, Decimal};
use cyber_std::{create_change_thought_block_msg, create_change_thought_input_msg, create_change_thought_period_msg, create_creat_thought_msg, create_create_energy_route_msg, create_cyberlink_msg, create_delete_energy_route_msg, create_edit_energy_route_name_msg, create_edit_energy_route_msg, create_forget_thought_msg, create_investmint_msg, BandwidthLoadResponse, BandwidthPriceResponse, BandwidthTotalResponse, CyberQuerier, CyberlinksAmountResponse, Link, Load, ThoughtLowestFeeResponse, NeuronBandwidthResponse, ParticleRankResponse, ParticlesAmountResponse, RouteResponse, RoutedEnergyResponse, RoutesResponse, ThoughtResponse, ThoughtStatsResponse, Trigger, Deps, DepsMut, Response, PoolParamsResponse, PoolLiquidityResponse, PoolSupplyResponse, PoolPriceResponse, PoolAddressResponse, create_create_pool_msg, create_deposit_within_batch_msg, create_withdraw_within_batch_msg, create_swap_within_batch_msg};

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
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Cyberlink { links } => cyberlink(deps, env, links),
        ExecuteMsg::Stake { validator, amount } => stake(deps, env, info, validator, amount),
        ExecuteMsg::Unstake { validator, amount } => unstake(deps, env, info, validator, amount),
        ExecuteMsg::Investmint {
            amount,
            resource,
            length,
        } => investmint(deps, env, info, amount, resource, length),
        ExecuteMsg::CreateEnergyRoute { destination, name } => {
            create_energy_route(deps, env, info, destination, name)
        }
        ExecuteMsg::EditEnergyRoute { destination, value } => {
            edit_energy_route(deps, env, info, destination, value)
        }
        ExecuteMsg::EditEnergyRouteName { destination, name } => {
            edit_energy_route_name(deps, env, info, destination, name)
        }
        ExecuteMsg::DeleteEnergyRoute { destination } => {
            delete_energy_route(deps, env, info, destination)
        }
        ExecuteMsg::CreateThought {
            trigger,
            load,
            name,
            particle,
        } => create_thought(deps, env, info, trigger, load, name, particle),
        ExecuteMsg::ForgetThought { name } => forget_thought(deps, env, info, name),
        ExecuteMsg::ChangeThoughtInput { name, input } => {
            change_thought_call_data(deps, env, info, name, input)
        }
        ExecuteMsg::ChangeThoughtPeriod { name, period } => {
            change_thought_period(deps, env, info, name, period)
        }
        ExecuteMsg::ChangeThoughtBlock { name, block } => {
            change_thought_block(deps, env, info, name, block)
        }
        ExecuteMsg::CreatePool { pool_type_id, deposit_coins } => {
            create_pool(deps, env, info, pool_type_id, deposit_coins)
        }
        ExecuteMsg::DepositWithinBatch { pool_id, deposit_coins } => {
            deposit_within_batch(deps, env, info, pool_id, deposit_coins)
        },
        ExecuteMsg::WithdrawWithinBatch { pool_id, pool_coin } => {
            withdraw_within_batch(deps, env, info, pool_id, pool_coin)
        },
        ExecuteMsg::SwapWithinBatch {
            pool_id,
            swap_type_id,
            offer_coin,
            demand_coin_denom,
            offer_coin_fee,
            order_price
        } => {
            swap_within_batch(
                deps,
                env,
                info,
                pool_id,
                swap_type_id,
                offer_coin,
                demand_coin_denom,
                offer_coin_fee,
                order_price
            )
        }
    }
}

pub fn cyberlink(
    _deps: DepsMut,
    env: Env,
    links: Vec<Link>,
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_cyberlink_msg(contract.into(), links);

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn stake(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator: String,
    amount: Coin,
) -> Result<Response, ContractError> {
    let amount = coin(u128::from(amount.amount), amount.denom);
    let res = Response::new().add_message(StakingMsg::Delegate {
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
) -> Result<Response, ContractError> {
    let amount = coin(u128::from(amount.amount), amount.denom);
    let res = Response::new().add_message(StakingMsg::Undelegate {
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
) -> Result<Response, ContractError> {
    let amount = coin(u128::from(amount.amount), amount.denom);
    let agent = env.contract.address;
    let msg = create_investmint_msg(agent.into(), amount.clone(), resource.into(), length.into());

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn create_energy_route(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination: String,
    name: String,
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_create_energy_route_msg(contract.into(), destination.into(), name.into());

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn edit_energy_route(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination: String,
    value: Coin,
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let value = coin(u128::from(value.amount), value.denom);
    let msg = create_edit_energy_route_msg(contract.into(), destination.into(), value.clone());

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn edit_energy_route_name(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination: String,
    name: String,
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_edit_energy_route_name_msg(contract.into(), destination.into(), name.into());

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn delete_energy_route(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination: String,
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_delete_energy_route_msg(contract.into(), destination.into());

    let res = Response::new().add_message(msg);
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
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_creat_thought_msg(
        contract.into(),
        trigger.into(),
        load.into(),
        name.into(),
        particle.into(),
    );

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn forget_thought(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_forget_thought_msg(contract.into(), name.into());

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn change_thought_call_data(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    name: String,
    input: String,
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_change_thought_input_msg(contract.into(), name.into(), input.into());

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn change_thought_period(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    name: String,
    period: u64,
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_change_thought_period_msg(contract.into(), name.into(), period.into());

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn change_thought_block(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    name: String,
    block: u64,
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_change_thought_block_msg(contract.into(), name.into(), block.into());

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn create_pool(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    pool_type_id: u32,
    deposit_coins: Vec<Coin>
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_create_pool_msg(
        contract.into(),
        pool_type_id,
        deposit_coins
    );

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn deposit_within_batch(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    pool_id: u64,
    deposit_coins: Vec<Coin>
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_deposit_within_batch_msg(
        contract.into(),
        pool_id,
        deposit_coins
    );

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn withdraw_within_batch(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    pool_id: u64,
    pool_coin: Coin
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_withdraw_within_batch_msg(
        contract.into(),
        pool_id,
        pool_coin
    );

    let res = Response::new().add_message(msg);
    Ok(res)
}

pub fn swap_within_batch(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    pool_id: u64,
    swap_type_id: u32,
    offer_coin: Coin,
    demand_coin_denom: String,
    offer_coin_fee: Coin,
    order_price: Decimal
) -> Result<Response, ContractError> {
    let contract = env.contract.address;
    let msg = create_swap_within_batch_msg(
        contract.into(),
        pool_id,
        swap_type_id,
        offer_coin,
        demand_coin_denom,
        offer_coin_fee,
        order_price
    );

    let res = Response::new().add_message(msg);
    Ok(res)
}

#[entry_point]
pub fn sudo(
    deps: DepsMut,
    env: Env,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::Heartbeat { beats } => do_beat(deps, env, beats),
        SudoMsg::Cyberlink { links } => cyberlink(deps, env, links),
        SudoMsg::Release {} => do_release(deps, env),
        SudoMsg::CpuLoop {} => do_cpu_loop(),
        SudoMsg::StorageLoop {} => do_storage_loop(deps),
        SudoMsg::MemoryLoop {} => do_memory_loop(),
        SudoMsg::Panic {} => do_panic(),
        SudoMsg::TransferFunds { recipient, amount } => {
            let response = Response::new().add_message(BankMsg::Send {
                to_address: recipient,
                amount,
            });
            Ok(response)
        }
    }
}

fn do_beat(
    deps: DepsMut,
    _env: Env,
    beats: u64,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;

    state.beats = state.beats + beats;

    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

fn do_release(
    deps: DepsMut,
    env: Env,
) -> Result<Response, ContractError> {
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

fn do_cpu_loop() -> Result<Response, ContractError> {
    let mut counter = 0u64;
    loop {
        counter += 1;
        if counter >= 9_000_000_000 {
            counter = 0;
        }
    }
}

fn do_storage_loop(
    deps: DepsMut,
) -> Result<Response, ContractError> {
    let mut test_case = 0u64;
    loop {
        deps.storage
            .set(b"test.key", test_case.to_string().as_bytes());
        test_case += 1;
    }
}

fn do_memory_loop() -> Result<Response, ContractError> {
    let mut data = vec![1usize];
    loop {
        // add one element
        data.push((*data.last().expect("must not be empty")) + 1);
    }
}

fn do_panic() -> Result<Response, ContractError> {
    panic!("This page intentionally faulted");
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ParticleRank { particle } => to_binary(&query_particle_rank(deps, particle)?),
        QueryMsg::ParticlesAmount {} => to_binary(&query_particles_amount(deps)?),
        QueryMsg::CyberlinksAmount {} => to_binary(&query_cyberlinks_amount(deps)?),
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
        QueryMsg::Thought { program, name } => to_binary(&query_thought(deps, program, name)?),
        QueryMsg::ThoughtStats { program, name } => {
            to_binary(&query_thought_stats(deps, program, name)?)
        }
        QueryMsg::ThoughtLowestFee {} => to_binary(&query_thought_lowest_fee(deps)?),
        QueryMsg::SourceRoutes { source } => to_binary(&query_source_routes(deps, source)?),
        QueryMsg::SourceRoutedEnergy { source } => {
            to_binary(&query_source_routed_energy(deps, source)?)
        }
        QueryMsg::DestinationRoutedEnergy { destination } => {
            to_binary(&query_destination_routed_energy(deps, destination)?)
        }
        QueryMsg::Route {
            source,
            destination,
        } => to_binary(&query_route(deps, source, destination)?),
        QueryMsg::BandwidthPrice {} => to_binary(&query_price(deps)?),
        QueryMsg::BandwidthLoad {} => to_binary(&query_load(deps)?),
        QueryMsg::BandwidthTotal {} => to_binary(&query_desirable_bandwidth(deps)?),
        QueryMsg::NeuronBandwidth { neuron } => to_binary(&query_neuron_bandwidth(deps, neuron)?),
        QueryMsg::PoolParams { pool_id } => to_binary(&query_pool_params(deps, pool_id)?),
        QueryMsg::PoolLiquidity { pool_id} => to_binary(&query_pool_liquidity(deps, pool_id)?),
        QueryMsg::PoolSupply { pool_id } => to_binary(&query_pool_supply(deps, pool_id)?),
        QueryMsg::PoolPrice { pool_id } => to_binary(&query_pool_price(deps, pool_id)?),
        QueryMsg::PoolAddress { pool_id } => to_binary(&query_pool_address(deps, pool_id)?),
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

pub fn query_particles_amount(deps: Deps) -> StdResult<ParticlesAmountResponse> {
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

pub fn query_thought_lowest_fee(deps: Deps) -> StdResult<ThoughtLowestFeeResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: ThoughtLowestFeeResponse = querier.query_thought_lowest_fee()?;

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

pub fn query_price(deps: Deps) -> StdResult<BandwidthPriceResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: BandwidthPriceResponse = querier.query_bandwidth_price()?;

    Ok(res)
}

pub fn query_load(deps: Deps) -> StdResult<BandwidthLoadResponse> {
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
    address: String,
) -> StdResult<NeuronBandwidthResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: NeuronBandwidthResponse = querier.query_neuron_bandwidth(address)?;

    Ok(res)
}

pub fn query_pool_params(
    deps: Deps,
    pool_id: u64,
) -> StdResult<PoolParamsResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: PoolParamsResponse = querier.query_pool_params(pool_id)?;

    Ok(res)
}

pub fn query_pool_liquidity(
    deps: Deps,
    pool_id: u64,
) -> StdResult<PoolLiquidityResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: PoolLiquidityResponse = querier.query_pool_liquidity(pool_id)?;

    Ok(res)
}

pub fn query_pool_supply(
    deps: Deps,
    pool_id: u64,
) -> StdResult<PoolSupplyResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: PoolSupplyResponse = querier.query_pool_supply(pool_id)?;

    Ok(res)
}

pub fn query_pool_price(
    deps: Deps,
    pool_id: u64,
) -> StdResult<PoolPriceResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: PoolPriceResponse = querier.query_pool_price(pool_id)?;

    Ok(res)
}

pub fn query_pool_address(
    deps: Deps,
    pool_id: u64,
) -> StdResult<PoolAddressResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let res: PoolAddressResponse = querier.query_pool_address(pool_id)?;

    Ok(res)
}
