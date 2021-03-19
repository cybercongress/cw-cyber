# Cyber Bindings for CosmWasm

Note: support in Bostrom's [bostromdev-1](https://github.com/cybercongress/go-cyber), cosmwasm [0.14.0-beta1](https://github.com/CosmWasm/cosmwasm/releases/tag/v0.14.0-beta1%2Bcontracts1) 

​
Note: not yet published to crates


This crate provides Cyber-specific bindings to enable your CosmWasm smart contracts to interact with the Cyber blockchain by exposing messages and queriers that can be emitted and used from within your contract.

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
use cyber_std::{ CyberQuerier, RankValueResponse };
​
...
​
// handler
pub fn try_something(
    deps: Deps,
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
use cyber_std::{create_cyberlink_msg, CyberMsgWrapper};
​
...
​
pub fn try_something(
    deps: DepsMut,
    env: Env,
    links: Vec<Link>
) -> Result<Response<CyberMsgWrapper>, Never> {
    ...
    ​let contract_addr = env.contract.address;
    let msg: CosmosMsg<CyberMsgWrapper> = create_cyberlink_msg(contract_addr, links);
    let res = HandleResponse {
        submessages: vec![],
        messages: vec![msg],
        log: vec![],
        data: None
    };
    Ok(res)
}
```

## Build

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="std_test_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.10.7
```

The optimized contracts are generated in the `artifacts/` directory.