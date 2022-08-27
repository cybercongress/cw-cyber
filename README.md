# Cyber Bindings for CosmWasm

![Crates.io](https://img.shields.io/crates/v/cyber-std)
![Crates.io](https://img.shields.io/crates/d/cyber-std)

This crate provides Cyber-specific bindings to enable your CosmWasm smart contracts to interact with the Cyber blockchain by exposing messages and queriers that can be emitted and used from within your contract.

## Bindings

Currently, the Cyber bindings include:

- Query support for:
  - Graph
    - ParticlesAmount
    - CyberlinksAmount
  - Bandwidth
    - BandwidthPrice
    - BandwidthLoad
    - BandwidthTotal
    - NeuronBandwidth
  - Rank
    - ParticleRank
  - Grid
    - SourceRoutes
    - SourceRoutedEnergy
    - DestinationRoutedEnergy
    - Route
  - DMN
    - Thought
    - ThoughtStats
    - ThoughtLowestFee
  - Liquidity
    - PoolParams
    - PoolLiquidity
    - PoolSupply
    - PoolPrice
    - PoolAddress
- Messages support for:
  - Graph
    - MsgCyberlink
  - Resources
    - MsgInvestmint
  - Grid
    - MsgCreateRoute
    - MsgEditRoute
    - MsgEditRouteName
    - MsgDeleteRoute
  - DMN
    - MsgCreateThought
    - MsgForgetThought
    - MsgChangeThoughtInput
    - MsgChangeThoughtPeriod
    - MsgChangeThoughtBlock
    - MsgChangeThoughtGasPrice
    - MsgChangeThoughtParticle
    - MsgChangeThoughtName
  - Liquidity
    - MsgCreatePool
    - MsgDepositWithinBath
    - MsgWithdrawWithinBath
    - MsgSwapWithinBath