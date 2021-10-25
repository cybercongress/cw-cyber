# Cyber Bindings for CosmWasm

Note: not yet published to crates


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
    - LowestFee
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