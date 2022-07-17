use std::ops::{Add, Mul};
use cosmwasm_std::{Decimal, StdError, Uint128};
use cosmwasm_std::{
    Addr, Deps, Env, Order, StdResult,
};
use cw1155::{ApprovedForAllResponse, BalanceResponse, BatchBalanceResponse, Expiration, TokensResponse};
use cw20_bonding::curves::{Curve, decimal, DecimalPlaces, SquareRoot};
use cw_storage_plus::Bound;
use crate::msg::{Fund, FundForNeuron, FundFromNeuron, FundsForNeuronResponse,
    FundsFromNeuronResponse, FundsResponse, NeuronVestingsResponse, SpotPriceResponse,
    SwapResponse, TokenId, TokenStateResponse, Vesting
};

use crate::state::{APPROVES, BALANCES, FUNDS_BY_BLOCKS, FUNDS_FOR_NEURONS, FUNDS_FROM_NEURONS, TOKENS, TOKENS_STATES, VESTINGS};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;

pub fn query_token_state(
    deps: Deps,
    _env: Env,
    token_id: TokenId,
) -> StdResult<TokenStateResponse> {
    let token_state = TOKENS_STATES.load(deps.storage, &token_id.clone())?;
    // TODO refactor response
    Ok(TokenStateResponse {
        reserve: token_state.reserve,
        supply: token_state.supply,
        funds: token_state.funds,
        funded: token_state.funded,
        reward: token_state.reward,
        locked: token_state.locked,
        created: token_state.created,
        init_price: token_state.init_price
    })
}

pub fn query_spot_price(
    deps: Deps,
    _env: Env,
    token_id: TokenId,
) -> StdResult<SpotPriceResponse> {
    let token_state = TOKENS_STATES.load(deps.storage, &token_id)?;
    let curve = SquareRoot::new(
        decimal(20u128, 2),
        DecimalPlaces::new(3, 3)
    );
    let spot_price = curve.spot_price(token_state.supply);
    Ok(SpotPriceResponse { spot_price })
}

pub fn query_balance(
    deps: Deps,
    _env: Env,
    owner_addr: Addr,
    token_id: TokenId,
) -> StdResult<BalanceResponse> {
    let balance = BALANCES
        .may_load(deps.storage, (&owner_addr, &token_id))?
        .unwrap_or_default();
    Ok(BalanceResponse { balance })
}

pub fn query_batch_balance(
    deps: Deps,
    _env: Env,
    owner_addr: Addr,
    token_ids: Vec<TokenId>,
) -> StdResult<BatchBalanceResponse> {
    let balances = token_ids
        .into_iter()
        .map(|token_id| -> StdResult<_> {
            Ok(BALANCES
                .may_load(deps.storage, (&owner_addr, &token_id))?
                .unwrap_or_default())
        })
        .collect::<StdResult<_>>()?;
    Ok(BatchBalanceResponse { balances })
}

pub fn query_all_approvals(
    deps: Deps,
    env: Env,
    owner: Addr,
    include_expired: bool,
    start_after: Option<Addr>,
    limit: Option<u32>,
) -> StdResult<ApprovedForAllResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(Bound::exclusive);

    let operators = APPROVES
        .prefix(&owner)
        .range(deps.storage, start, None, Order::Ascending)
        .filter(|r| include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block))
        .take(limit)
        .map(build_approval)
        .collect::<StdResult<_>>()?;
    Ok(ApprovedForAllResponse { operators })
}

pub fn query_tokens(
    deps: Deps,
    owner: Addr,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));

    let tokens = BALANCES
        .prefix(&owner)
        .keys(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect::<StdResult<_>>()?;
    Ok(TokensResponse { tokens })
}

pub fn query_all_tokens(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));
    let tokens = TOKENS
        .keys(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect::<StdResult<_>>()?;
    Ok(TokensResponse { tokens })
}

pub fn build_approval(item: StdResult<(Addr, Expiration)>) -> StdResult<cw1155::Approval> {
    item.map(|(addr, expires)| cw1155::Approval {
        spender: addr.into(),
        expires,
    })
}

pub fn query_all_funds(
    deps: Deps,
    _env: Env,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<FundsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let funds = FUNDS_BY_BLOCKS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(build_fund)
        .collect::<StdResult<_>>()?;
    Ok(FundsResponse { funds })
}

pub fn build_fund(item: StdResult<(u64, String)>) -> StdResult<Fund> {
    item.map(|(height, token_id)| Fund {
        token_id,
        height
    })
}

pub fn query_all_funds_from_neuron(
    deps: Deps,
    _env: Env,
    neuron: Addr,
) -> StdResult<FundsFromNeuronResponse> {
    let funds = FUNDS_FROM_NEURONS
        .prefix(&neuron)
        .range(deps.storage, None, None, Order::Ascending)
        .map(build_funds_from)
        .collect::<StdResult<_>>()?;
    Ok(FundsFromNeuronResponse { funds })
}

pub fn build_funds_from(item: StdResult<(String, Uint128)>) -> StdResult<FundFromNeuron> {
    item.map(|(token_id, amount)| FundFromNeuron {
        token_id,
        amount,
    })
}

pub fn query_all_funds_for_neuron(
    deps: Deps,
    _env: Env,
    token_id: TokenId,
) -> StdResult<FundsForNeuronResponse> {
    let funds = FUNDS_FOR_NEURONS
        .prefix(&token_id)
        .range(deps.storage, None, None, Order::Ascending)
        .map(build_funds_for)
        .collect::<StdResult<_>>()?;
    Ok(FundsForNeuronResponse { funds })
}

pub fn build_funds_for(item: StdResult<(Addr, Uint128)>) -> StdResult<FundForNeuron> {
    item.map(|(addr, amount)| FundForNeuron {
        address: addr.into(),
        amount,
    })
}

pub fn query_all_neuron_vestings(
    deps: Deps,
    _env: Env,
    neuron: Addr,
) -> StdResult<NeuronVestingsResponse> {
    let vestings = VESTINGS
        .prefix(&neuron)
        .range(deps.storage, None, None, Order::Ascending)
        .map(build_vesting)
        .collect::<StdResult<_>>()?;
    Ok(NeuronVestingsResponse { vestings })
}

pub fn build_vesting(item: StdResult<(String, Uint128)>) -> StdResult<Vesting> {
    item.map(|(token_id, amount)| Vesting {
        token_id,
        amount,
    })
}

pub fn query_swap_out_in(
    deps: Deps,
    _env: Env,
    from: TokenId,
    to: TokenId,
    value: Uint128,
) -> StdResult<SwapResponse> {
    let mut out_token = TOKENS_STATES.load(deps.storage, &from.clone())?;
    let mut in_token = TOKENS_STATES.load(deps.storage, &to.clone())?;

    let curve = SquareRoot::new(
        decimal(20u128, 2),
        DecimalPlaces::new(3,3)
    );

    out_token.supply = out_token
        .supply
        .checked_sub(value)
        .map_err(StdError::overflow)?;
    let new_reserve = curve.reserve(out_token.supply);
    let mut released_out = out_token
        .reserve
        .checked_sub(new_reserve)
        .map_err(StdError::overflow)?;

    let reward_out = Decimal::percent(out_token.reward).mul(released_out.clone());
    released_out = released_out
        .checked_sub(reward_out.clone())
        .map_err(StdError::overflow)?;

    let reward_in = Decimal::percent(in_token.reward).mul(released_out.clone());
    let buy_in = released_out
        .checked_sub(reward_in.clone())
        .map_err(StdError::overflow)?;

    in_token.reserve += buy_in;
    let new_supply = curve.supply(in_token.reserve);
    let minted = new_supply
        .checked_sub(in_token.supply)
        .map_err(StdError::overflow)?;

    Ok(SwapResponse {
        from,
        to,
        sell: value,
        buy: minted
    })
}

pub fn query_swap_in_out(
    deps: Deps,
    _env: Env,
    to: TokenId,
    from: TokenId,
    value: Uint128,
) -> StdResult<SwapResponse> {
    let mut out_token = TOKENS_STATES.load(deps.storage, &from.clone())?;
    let in_token = TOKENS_STATES.load(deps.storage, &to.clone())?;

    let curve = SquareRoot::new(
        decimal(20u128, 2),
        DecimalPlaces::new(3,3)
    );

    let mut reserve_in = curve.reserve(in_token.supply.add(value));
    reserve_in = reserve_in
        .checked_sub(in_token.reserve)
        .map_err(StdError::overflow)?;
    let reward_in = Decimal::percent(in_token.reward).mul(reserve_in.clone());
    reserve_in = reserve_in
        .checked_add(reward_in)
        .map_err(StdError::overflow)?;

    let reward_out = Decimal::percent(out_token.reward).mul(reserve_in.clone());
    let reserve_out = reserve_in
        .checked_sub(reward_out)
        .map_err(StdError::overflow)?;

    out_token.reserve -= reserve_out;
    let new_supply = curve.supply(out_token.reserve);
    let burned = out_token.supply
        .checked_sub(new_supply)
        .map_err(StdError::overflow)?;

    Ok(SwapResponse {
        from,
        to,
        sell: burned,
        buy: value
    })
}