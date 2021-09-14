# Cyber Bindings for CosmWasm

Note: not yet published to crates


This crate provides Cyber-specific bindings to enable your CosmWasm smart contracts to interact with the Cyber blockchain by exposing messages and queriers that can be emitted and used from within your contract.

## Bindings

Currently, the Cyber bindings include:

- Query support for:
  - Graph
    - CidsCount
    - LinksCount
  - Bandwidth
    - Price
    - Load
    - DesirableBandwidth
    - AccountBandwidth
  - Rank
    - RankValueByCid
  - Energy
    - SourceRoutes
    - SourceRoutedEnergy
    - DestinationRoutedEnergy
    - Route
  - Cron
    - Job
    - JobStats
    - GetLowestFee
- Messages support for:
  - Graph
    - MsgCyberlink
  - Resources
    - MsgInvestmint
  - Energy
    - MsgCreateRoute
    - MsgEditRoute
    - MsgEditRouteAlias
    - MsgDeleteRoute
  - Cron
    - MsgAddJob
    - MsgRemoveJob
    - MsgChangeJobCallData
    - MsgChangeJobPeriod
    - MsgChangeJobBlock