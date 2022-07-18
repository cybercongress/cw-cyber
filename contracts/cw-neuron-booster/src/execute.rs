use std::ops::{Add, Div, Mul, Sub};

use cosmwasm_std::{BankMsg, coins, Decimal, StdError, Storage};
use cosmwasm_std::{
    Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg,
    Uint128,
};
use cw1155::{
    ApproveAllEvent,
    Cw1155BatchReceiveMsg, Cw1155ReceiveMsg, Expiration,
    TokenId, TransferEvent,
};
use cw20_bonding::curves::{Curve, decimal, DecimalPlaces, SquareRoot};
use cw_utils::{Event, must_pay};

use crate::error::ContractError;
use crate::state::{
    APPROVES, BALANCES, FUNDS_BY_BLOCKS, FUNDS_FOR_NEURONS,
    FUNDS_FROM_NEURONS, MINTER, TOKENS, TOKENS_STATES, TokenState, VESTINGS
};

const RESERVE_DENOM: &str = "milliampere";
const FUND_PERIOD: u64 = 250;
const VESTING_PERIOD: u64 = 500;

// TODO apply fee to buy and sell to self?
// TODO if reward rounded to 0?

pub fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    reward: u64,
    locked: bool,
    msg: Option<Binary>,
) -> Result<Response, ContractError> {
    let token_id = info.clone().sender.into_string();

    if reward > 100 { return Err(ContractError::Unauthorized {}) };

    let payment = must_pay(&info.clone(), RESERVE_DENOM)?;

    let token_state = TokenState{
        reserve:Uint128::new(0),
        supply: Uint128::new(0),
        funds: payment,
        funded: false,
        reward,
        locked,
        created: env.block.height,
        init_price: Decimal::zero()
    };

    TOKENS_STATES.save(deps.storage, &token_id.clone(), &token_state)?;
    FUNDS_BY_BLOCKS.save(deps.storage, env.block.height, &token_id)?;
    FUNDS_FROM_NEURONS.save(deps.storage, (&info.sender, &token_id), &payment)?;
    FUNDS_FOR_NEURONS.save(deps.storage,(&token_id, &info.sender), &payment)?;

    let sub_info = MessageInfo {
        sender: env.clone().contract.address,
        funds: vec![],
    };

    _execute_mint(deps, env, sub_info, info.clone().sender.to_string(), token_id.clone(), Uint128::new(0), msg)?;

    let res = Response::new()
        .add_attribute("action", "mint")
        .add_attribute("token_id", token_id)
        .add_attribute("from", info.sender)
        .add_attribute("reward", Decimal::percent(reward).to_string())
        .add_attribute("locked", locked.to_string())
        .add_attribute("funds", payment.to_string());
    Ok(res)
}

pub fn execute_fund(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: TokenId
) -> Result<Response, ContractError> {
    let mut token_state = TOKENS_STATES.load(deps.storage, &token_id)?;

    if token_state.created.add(FUND_PERIOD) < env.block.height {
        return Err(ContractError::FundingEnded {})
    }

    let payment = must_pay(&info.clone(), RESERVE_DENOM)?;

    token_state.funds = token_state.funds.add(payment);
    TOKENS_STATES.save(deps.storage, &token_id, &token_state)?;

    FUNDS_FROM_NEURONS.update(
        deps.storage,
        (&info.sender, &token_id),
        |funds: Option<Uint128>| -> StdResult<_> {
            match funds {
                Some(amount) => Ok(amount.add(payment)),
                None => Ok(payment),
            }
        },
    )?;

    FUNDS_FOR_NEURONS.update(
        deps.storage,
        (&token_id, &info.sender),
        |funds: Option<Uint128>| -> StdResult<_> {
            match funds {
                Some(amount) => Ok(amount.add(payment)),
                None => Ok(payment),
            }
        },
    )?;

    let sub_info = MessageInfo {
        sender: env.clone().contract.address,
        funds: vec![],
    };

    _execute_mint(deps, env, sub_info, info.sender.to_string(), token_id.clone(), Uint128::new(0), None)?;

    let res = Response::new()
        .add_attribute("action", "fund")
        .add_attribute("token_id", token_id)
        .add_attribute("from", info.sender)
        .add_attribute("funds", payment.to_string());
    Ok(res)
}

pub fn execute_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: TokenId,
) -> Result<Response, ContractError> {
    let mut token_state = TOKENS_STATES.load(deps.storage, &token_id)?;

    if token_state.created.add(FUND_PERIOD) > env.block.height {
        return Err(ContractError::FundingPeriod {})
    }

    if VESTINGS.has(deps.storage, (&info.sender, &token_id)) {
        return Err(ContractError::FundsClaimed {})
    }

    if !token_state.funded {
        TOKENS_STATES.update(
            deps.storage,
            &token_id,
            |ts: Option<TokenState>| -> StdResult<_> {
                let mut state = ts.unwrap();
                let curve = SquareRoot::new(decimal(20u128, 2), DecimalPlaces::new(3, 3));
                state.reserve = state.funds;
                state.supply = curve.supply(state.reserve);
                state.funded = true;
                state.init_price = Decimal::from_ratio(state.reserve, state.supply);
                token_state = state.clone();
                Ok(state)
            },
        )?;
    }

    let neuron_funds = FUNDS_FROM_NEURONS.load(deps.storage, (&info.sender, &token_id))?;
    let amount = Decimal::new(neuron_funds)
        .div(token_state.init_price)
        .atomics();

    let sub_info = MessageInfo {
        sender: env.clone().contract.address,
        funds: vec![],
    };

    VESTINGS.save(deps.storage, (&info.sender, &token_id), &amount)?;

    _execute_mint(deps, env, sub_info, info.sender.to_string(), token_id.clone(), amount, None)?;

    let res = Response::new()
        .add_attribute("action", "claim")
        .add_attribute("token_id", token_id.clone())
        .add_attribute("amount", amount.to_string());

    Ok(res)
}

pub fn execute_buy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: TokenId,
    msg: Option<Binary>,
) -> Result<Response, ContractError> {
    let mut token_state = TOKENS_STATES.load(deps.storage, &token_id)?;

    if token_state.created.add(FUND_PERIOD) > env.block.height {
        return Err(ContractError::FundingPeriod {})
    }

    if !token_state.funded { return Err(ContractError::FundingPeriod {}) }

    let payment = must_pay(&info.clone(), RESERVE_DENOM)?;

    let reward = Decimal::percent(token_state.reward).mul(payment);
    let curve = SquareRoot::new(decimal(20u128, 2), DecimalPlaces::new(3, 3));

    let payment_to = payment
        .checked_sub(reward)
        .map_err(StdError::overflow)?;
    token_state.reserve += payment_to;
    let new_supply = curve.supply(token_state.reserve);
    let minted = new_supply
        .checked_sub(token_state.supply)
        .map_err(StdError::overflow)?;
    token_state.supply = new_supply;
    TOKENS_STATES.save(deps.storage, &token_id.clone(), &token_state)?;

    let sub_info = MessageInfo {
        sender: env.clone().contract.address,
        funds: vec![],
    };

    _execute_mint(deps, env, sub_info, info.clone().sender.to_string(), token_id.clone(), minted, msg)?;

    let res = Response::new()
        .add_message(BankMsg::Send {
            to_address: token_id.clone(),
            amount: coins(reward.u128(), RESERVE_DENOM)})
        .add_attribute("action", "buy")
        .add_attribute("token_id", token_id)
        .add_attribute("from", info.sender)
        .add_attribute("reserve", token_state.reserve.to_string())
        .add_attribute("supply", token_state.supply.to_string())
        .add_attribute("payment", payment.to_string())
        .add_attribute("reward", reward.to_string())
        .add_attribute("minted", minted.to_string());
    Ok(res)
}

pub fn execute_sell(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    token_id: TokenId,
    amount: Uint128
) -> Result<Response, ContractError> {
    _execute_burn(deps.branch(), env.clone(), info.clone(), from, token_id.clone(), amount)?;

    let mut token_state = TOKENS_STATES.load(deps.storage, &token_id)?;

    if token_state.created.add(FUND_PERIOD) > env.block.height {
        return Err(ContractError::FundingPeriod {})
    }

    if !token_state.funded {
        return Err(ContractError::FundingPeriod {})
    }

    let curve = SquareRoot::new(decimal(20u128, 2), DecimalPlaces::new(3,3));
    token_state.supply = token_state
        .supply
        .checked_sub(amount)
        .map_err(StdError::overflow)?;
    let new_reserve = curve.reserve(token_state.supply);
    let mut released = token_state
        .reserve
        .checked_sub(new_reserve)
        .map_err(StdError::overflow)?;
    token_state.reserve = new_reserve;
    TOKENS_STATES.save(deps.storage, &token_id, &token_state)?;

    let reward = Decimal::percent(token_state.reward).mul(released);
    released = released.sub(reward);

    let res = Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(released.u128(), RESERVE_DENOM),
        })
        .add_message(BankMsg::Send {
            to_address: token_id.clone(),
            amount: coins(reward.u128(), RESERVE_DENOM),
        })
        .add_attribute("action", "burn")
        .add_attribute("token_id", token_id)
        .add_attribute("from", info.sender)
        .add_attribute("reward", reward)
        .add_attribute("payment", released)
        .add_attribute("reserve", token_state.reserve)
        .add_attribute("supply", token_state.supply);

    Ok(res)
}

pub fn execute_lock(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: TokenId,
) -> Result<Response, ContractError> {
    let mut token_state = TOKENS_STATES.load(deps.storage, &token_id)?;

    if token_id.ne(&info.sender.to_string()) {
        return Err(ContractError::Unauthorized {})
    }

    if token_state.created.add(FUND_PERIOD) > env.block.height {
        return Err(ContractError::FundingPeriod {})
    }

    if !token_state.funded {
        return Err(ContractError::FundingPeriod {})
    }

    token_state.locked = true;
    TOKENS_STATES.save(deps.storage, &token_id, &token_state)?;

    let res = Response::new()
        .add_attribute("action", "lock")
        .add_attribute("token_id", token_id);

    Ok(res)
}

pub fn execute_update_reward(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: TokenId,
    reward: u64,
) -> Result<Response, ContractError> {
    let mut token_state = TOKENS_STATES.load(deps.storage, &token_id)?;

    if token_id.ne(&info.sender.to_string()) {
        return Err(ContractError::Unauthorized {})
    }

    if token_state.created.add(FUND_PERIOD) > env.block.height {
        return Err(ContractError::FundingPeriod {})
    }

    if !token_state.funded {
        return Err(ContractError::FundingPeriod {})
    }

    if token_state.locked {
        return Err(ContractError::TokenLocked {})
    }

    token_state.reward = reward;
    TOKENS_STATES.save(deps.storage, &token_id, &token_state)?;

    let res = Response::new()
        .add_attribute("action", "update_reward")
        .add_attribute("token_id", token_id)
        .add_attribute("reward", reward.to_string());

    Ok(res)
}

pub fn execute_swap_out_in(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: TokenId,
    to: TokenId,
    value: Uint128,
) -> Result<Response, ContractError> {
    _execute_burn(deps.branch(), env.clone(), info.clone(), info.sender.to_string(), from.clone(), value)?;

    let mut out_token = TOKENS_STATES.load(deps.storage, &from)?;

    if out_token.created.add(FUND_PERIOD) > env.block.height {
        return Err(ContractError::FundingPeriod {})
    }
    if !out_token.funded { return Err(ContractError::FundingPeriod {}) }

    let mut in_token = TOKENS_STATES.load(deps.storage, &to)?;

    if in_token.created.add(FUND_PERIOD) > env.block.height {
        return Err(ContractError::FundingPeriod {})
    }
    if !in_token.funded { return Err(ContractError::FundingPeriod {}) }

    let curve = SquareRoot::new(
        decimal(20u128, 2),
        DecimalPlaces::new(3,3)
    );

    // sell out with value
    out_token.supply = out_token
        .supply
        .checked_sub(value)
        .map_err(StdError::overflow)?;
    let new_reserve = curve.reserve(out_token.supply);
    let mut released_out = out_token
        .reserve
        .checked_sub(new_reserve)
        .map_err(StdError::overflow)?;
    out_token.reserve = new_reserve;
    TOKENS_STATES.save(deps.storage, &from, &out_token)?;

    let reward_out = Decimal::percent(out_token.reward).mul(released_out);
    released_out = released_out.sub(reward_out);

    let reward_in = Decimal::percent(in_token.reward).mul(released_out);
    let buy_in = released_out.sub(reward_in);

    // buy in with amount
    in_token.reserve += buy_in;
    let new_supply = curve.supply(in_token.reserve);
    let minted = new_supply
        .checked_sub(in_token.supply)
        .map_err(StdError::overflow)?;
    in_token.supply = new_supply;
    TOKENS_STATES.save(deps.storage, &to, &in_token)?;

    let sub_info = MessageInfo {
        sender: env.clone().contract.address,
        funds: vec![],
    };

    _execute_mint(deps, env, sub_info, info.clone().sender.to_string(), to.clone(), minted, None)?;

    let res = Response::new()
        .add_message(BankMsg::Send {
            to_address: from.clone(),
            amount: coins(reward_out.u128(), RESERVE_DENOM)})
        .add_message(BankMsg::Send {
            to_address: to.clone(),
            amount: coins(reward_in.u128(), RESERVE_DENOM)})
        .add_attribute("action", "swap_out_in")
        .add_attribute("addr", info.sender)
        .add_attribute("from", from.to_string())
        .add_attribute("to", to.to_string())
        .add_attribute("sell", value.to_string())
        .add_attribute("bought", minted.to_string())
        .add_attribute("reward_out", reward_out.to_string())
        .add_attribute("reward_in", reward_in.to_string());

    Ok(res)
}

pub fn execute_swap_in_out(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to: TokenId,
    from: TokenId,
    value: Uint128,
) -> Result<Response, ContractError> {
    let mut out_token = TOKENS_STATES.load(deps.storage, &from)?;

    if out_token.created.add(FUND_PERIOD) > env.block.height {
        return Err(ContractError::FundingPeriod {})
    }
    if !out_token.funded { return Err(ContractError::FundingPeriod {}) }

    let mut in_token = TOKENS_STATES.load(deps.storage, &to)?;

    if in_token.created.add(FUND_PERIOD) > env.block.height {
        return Err(ContractError::FundingPeriod {})
    }
    if !in_token.funded { return Err(ContractError::FundingPeriod {}) }

    let curve = SquareRoot::new(
        decimal(20u128, 2),
        DecimalPlaces::new(3,3)
    );

    let mut reserve_in = curve.reserve(in_token.supply.add(value));
    // THIS LINE!!!
    reserve_in = reserve_in.sub(in_token.reserve);
    let reward_in = Decimal::percent(in_token.reward).mul(reserve_in);
    reserve_in = reserve_in.add(reward_in);

    let reward_out = Decimal::percent(out_token.reward).mul(reserve_in);
    let reserve_out = reserve_in.add(reward_out);

    out_token.reserve -= reserve_out;
    let new_supply = curve.supply(out_token.reserve);
    // THIS LINE!!!
    let burned = out_token.supply
        .checked_sub(new_supply)
        .map_err(StdError::overflow)?;
    out_token.supply = new_supply;
    TOKENS_STATES.save(deps.storage, &from, &out_token)?;

    _execute_burn(deps.branch(), env.clone(), info.clone(), info.sender.to_string(), from.clone(), burned.clone())?;

    //----

    in_token.reserve += reserve_in;
    let new_supply = curve.supply(in_token.reserve);
    let minted = new_supply
        .checked_sub(in_token.supply)
        .map_err(StdError::overflow)?;
    in_token.supply = new_supply;
    TOKENS_STATES.save(deps.storage, &to, &in_token)?;

    let sub_info = MessageInfo {
        sender: env.clone().contract.address,
        funds: vec![],
    };

    _execute_mint(deps, env, sub_info, info.sender.to_string(), to.clone(), minted.clone(), None)?;

    let res = Response::new()
        .add_message(BankMsg::Send {
            to_address: from.clone(),
            amount: coins(reward_out.u128(), RESERVE_DENOM)})
        .add_message(BankMsg::Send {
            to_address: to.clone(),
            amount: coins(reward_out.u128(), RESERVE_DENOM)})
        .add_attribute("action", "swap_in_out")
        .add_attribute("addr", info.sender)
        .add_attribute("from", from.to_string())
        .add_attribute("to", to.to_string())
        .add_attribute("bought", value.to_string())
        .add_attribute("sell", burned.to_string())
        .add_attribute("reward_out", reward_out.to_string())
        .add_attribute("reward_in", reward_in.to_string());

    Ok(res)

}

/// returns true iff the sender can execute approve or reject on the contract
pub fn check_can_approve(
    deps: Deps,
    env: &Env,
    owner: &Addr,
    operator: &Addr
) -> StdResult<bool> {
    // owner can approve
    if owner == operator {
        return Ok(true);
    }
    // operator can approve
    let op = APPROVES.may_load(deps.storage, (owner, operator))?;
    Ok(match op {
        Some(ex) => !ex.is_expired(&env.block),
        None => false,
    })
}

pub fn execute_send_from(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    to: String,
    token_id: TokenId,
    amount: Uint128,
    msg: Option<Binary>,
) -> Result<Response, ContractError> {
    let from_addr = deps.api.addr_validate(&from)?;
    let to_addr = deps.api.addr_validate(&to)?;

    _guard_can_approve(deps.as_ref(), &env, &from_addr, &info.sender)?;

    let mut rsp = Response::default();

    let event = _execute_transfer_inner(
        &mut deps,
        env.block.height,
        Some(&from_addr),
        Some(&to_addr),
        &token_id,
        amount,
    )?;
    event.add_attributes(&mut rsp);

    if let Some(msg) = msg {
        rsp.messages = vec![SubMsg::new(
            Cw1155ReceiveMsg {
                operator: info.sender.to_string(),
                from: Some(from),
                amount,
                token_id: token_id.clone(),
                msg,
            }
                .into_cosmos_msg(to)?,
        )]
    }

    Ok(rsp)
}

pub fn execute_batch_send_from(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    to: String,
    batch: Vec<(TokenId, Uint128)>,
    msg: Option<Binary>,
) -> Result<Response, ContractError> {
    let from_addr = deps.api.addr_validate(&from)?;
    let to_addr = deps.api.addr_validate(&to)?;

    _guard_can_approve(deps.as_ref(), &env, &from_addr, &info.sender)?;

    let mut rsp = Response::default();
    for (token_id, amount) in batch.iter() {
        let event = _execute_transfer_inner(
            &mut deps,
            env.block.height,
            Some(&from_addr),
            Some(&to_addr),
            token_id,
            *amount,
        )?;
        event.add_attributes(&mut rsp);
    }

    if let Some(msg) = msg {
        rsp.messages = vec![SubMsg::new(
            Cw1155BatchReceiveMsg {
                operator: info.sender.to_string(),
                from: Some(from),
                batch,
                msg,
            }
                .into_cosmos_msg(to)?,
        )]
    };

    Ok(rsp)
}

pub fn execute_approve_all(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operator: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    // reject expired data as invalid
    let expires = expires.unwrap_or_default();
    if expires.is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }

    // set the operator for us
    let operator_addr = deps.api.addr_validate(&operator)?;
    APPROVES.save(deps.storage, (&info.sender, &operator_addr), &expires)?;

    let mut rsp = Response::default();
    ApproveAllEvent {
        sender: info.sender.as_ref(),
        operator: &operator,
        approved: true,
    }
        .add_attributes(&mut rsp);
    Ok(rsp)
}

pub fn execute_revoke_all(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    operator: String
) -> Result<Response, ContractError> {
    let operator_addr = deps.api.addr_validate(&operator)?;
    APPROVES.remove(deps.storage, (&info.sender, &operator_addr));

    let mut rsp = Response::default();
    ApproveAllEvent {
        sender: info.sender.as_ref(),
        operator: &operator,
        approved: false,
    }
        .add_attributes(&mut rsp);
    Ok(rsp)
}

/// When from is None: mint new coins
/// When to is None: burn coins
/// When both are None: no token balance is changed, pointless but valid
///
/// Make sure permissions are checked before calling this.
fn _execute_transfer_inner<'a>(
    deps: &'a mut DepsMut,
    height: u64,
    from: Option<&'a Addr>,
    to: Option<&'a Addr>,
    token_id: &'a str,
    amount: Uint128,
) -> Result<TransferEvent<'a>, ContractError> {
    if let Some(from_addr) = from {

        _deduct_vesting(deps.storage, &from_addr, (&token_id).to_string(), amount, height)?;

        BALANCES.update(
            deps.storage,
            (from_addr, token_id),
            |balance: Option<Uint128>| -> StdResult<_> {
                Ok(balance.unwrap_or_default().checked_sub(amount)?)
            },
        )?;
    }

    if let Some(to_addr) = to {
        BALANCES.update(
            deps.storage,
            (to_addr, token_id),
            |balance: Option<Uint128>| -> StdResult<_> {
                Ok(balance.unwrap_or_default().checked_add(amount)?)
            },
        )?;
    }

    Ok(TransferEvent {
        from: from.map(|x| x.as_ref()),
        to: to.map(|x| x.as_ref()),
        token_id,
        amount,
    })
}

fn _execute_mint(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to: String,
    token_id: TokenId,
    amount: Uint128,
    msg: Option<Binary>,
) -> Result<Response, ContractError> {
    let to_addr = deps.api.addr_validate(&to)?;

    if info.sender != MINTER.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    let mut rsp = Response::default();

    let event = _execute_transfer_inner(&mut deps, env.block.height, None, Some(&to_addr), &token_id, amount)?;
    event.add_attributes(&mut rsp);

    if let Some(msg) = msg {
        rsp.messages = vec![SubMsg::new(
            Cw1155ReceiveMsg {
                operator: info.sender.to_string(),
                from: None,
                amount,
                token_id: token_id.clone(),
                msg,
            }
                .into_cosmos_msg(to)?,
        )]
    }

    // insert if not exist
    if !TOKENS.has(deps.storage, &token_id) {
        // we must save some valid data here
        TOKENS.save(deps.storage, &token_id, &String::new())?;
    }

    Ok(rsp)
}

fn _execute_burn(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: String,
    token_id: TokenId,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let from_addr = deps.api.addr_validate(&from)?;

    // whoever can transfer these tokens can burn
    _guard_can_approve(deps.as_ref(), &env, &from_addr, &info.sender)?;

    let mut rsp = Response::default();
    let event = _execute_transfer_inner(&mut deps, env.block.height,Some(&from_addr), None, &token_id, amount)?;
    event.add_attributes(&mut rsp);
    Ok(rsp)
}

fn _guard_can_approve(
    deps: Deps,
    env: &Env,
    owner: &Addr,
    operator: &Addr,
) -> Result<(), ContractError> {
    if !check_can_approve(deps, env, owner, operator)? {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(())
    }
}

fn _deduct_vesting(
    storage: &mut dyn Storage,
    address: &Addr,
    token_id: TokenId,
    amount: Uint128,
    current_block: u64,
) -> Result<(), ContractError> {
    if !VESTINGS.has(storage, (address, &token_id)) {
        return Ok(())
    };

    let vesting = VESTINGS.load(storage, (address, &token_id))?;
    if vesting.is_zero() { return Ok(()) };

    let balance = BALANCES.load(storage, (address, &token_id))?;
    let token_state = TOKENS_STATES.load(storage, &token_id)?;

    if token_state.created.add(VESTING_PERIOD) > current_block {
        return if balance.sub(amount) < vesting {
            Err(ContractError::VestingPeriod {})
        } else { Ok(()) }
    } else {
        VESTINGS.update(storage, (address, &token_id), |vesting| {
            match vesting {
                Some(vesting_amount) => {
                    if vesting_amount > amount {
                        Ok(vesting_amount.sub(amount))
                    } else { Ok(Uint128::zero()) }
                }
                None => Err(ContractError::Unauthorized {}),
            }
        })?;
    }

    Ok(())
}