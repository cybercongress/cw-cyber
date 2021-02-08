# Cyber Bindings for CosmWasm
​
This crate provides Cyber-specific bindings to enable your CosmWasm smart contracts to interact with the Cyber blockchain by exposing messages and queriers that can be emitted and used from within your contract.
​
## Installation
​
Add the following to your smart contract's `Cargo.toml`:
​
```toml
[dependencies]
cyber-cosmwasm = { version = "0.7.0" }
```
​
## Contents
​
Currently, the Cyber bindings include:
​
- Query support for:
  - Graph
    - cyberlinks count
    - cids count
  - Rank
    - rank value by cid
    - rank value by id
- Messages
  - Graph 
    - MsgCyberlink
​
## Usage
​
### Querying
​
In order to use the query functions enabled by the bindings, create a `CyberQuerier` instance within your contract logic -- in either `init()`, `handle()`, or `query()` entrypoints. You can access all the enabled queries through this object.
​
```rust
// src/contract.rs
use cosmwasm_std::Coin;
use cyber_cosmwasm::{ CyberQuerier, RankValueResponse};
​
...
​
// handler
pub fn try_something<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    cid: String
) -> StdResult<HandleResponse> {
    let querier = CyberQuerier::new(&deps.querier);
    let rank_value: RankValueResponse = querier.query_rank_value_by_cid(cid)?;
    ...
}
```
​
## Creating Messages
​
**NOTE:** The Cyber bindings do not cover messages that have already been implemented by the CosmWasm team, such as staking-related messages and fundamental ones like `MsgSend`.
​
You may want your contract to perform messages such as `MsgCyberlink` operations at the end of its execution. To do this, create a message using the predefined functions:
​
- `create_cyberlink_msg`
​
And add it to the vector of `messages` in your `HandleResponse` before you return `Ok`.
​
```rust
use cosmwasm_std::CosmosMsg;
use cyber_cosmwasm::{create_cyberlink_msg, CyberMsgWrapper};
​
...
​
pub fn try_something<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    links: Vec<Link>
) -> StdResult<HandleResponse<CyberMsgWrapper>> {
    ...
    ​let contract_addr = env.contract.address;
    let msg: CosmosMsg<CyberMsgWrapper> = create_cyberlink_msg(contract_addr, links);
    let res = HandleResponse {
        messages: vec![msg],
        log: vec![],
        data: None
    };
    Ok(res)
}
```