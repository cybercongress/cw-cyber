# Cyber Bindings for CosmWasm

![Crates.io](https://img.shields.io/crates/v/cyber-std)
![Crates.io](https://img.shields.io/crates/d/cyber-std)

This crate provides Cyber-specific bindings to enable your CosmWasm smart contracts to interact with the Cyber blockchain by exposing messages and queriers that can be emitted and used from within your contract.

## Bindings

Currently, the Cyber bindings include:

| Module    	| Execute                                                                                                                                                                                          	| Query                                                                  	|
|-----------	|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------	|------------------------------------------------------------------------	|
| Graph     	| MsgCyberlink                                                                                                                                                                                     	| CyberlinkExist<br>ParticleExist<br>ParticlesAmount<br>CyberlinksAmount 	|
| Rank      	|                                                                                                                                                                                                  	| ParticleRank                                                           	|
| Bandwidth 	|                                                                                                                                                                                                  	| BandwidthPrice<br>BandwidthLoad<br>BandwidthTotal<br>NeuronBandwidth   	|
| Resources 	| MsgInvestmint                                                                                                                                                                                    	|                                                                        	|
| Grid      	| MsgCreateRoute<br>MsgEditRoute<br>MsgEditRouteName<br>MsgDeleteRoute                                                                                                                             	| SourceRoutes<br>SourceRoutedEnergy<br>DestinationRoutedEnergy<br>Route 	|
| DMN       	| MsgCreateThought<br>MsgForgetThought<br>MsgChangeThoughtInput<br>MsgChangeThoughtPeriod<br>MsgChangeThoughtBlock<br>MsgChangeThoughtGasPrice<br>MsgChangeThoughtParticle<br>MsgChangeThoughtName 	| Thought<br>ThoughtStats<br>ThoughtLowestFee                            	|
| Liquidity 	| MsgCreatePool<br>MsgDepositWithinBath<br>MsgWithdrawWithinBath<br>MsgSwapWithinBath                                                                                                              	| PoolParams<br>PoolLiquidity<br>PoolSupply<br>PoolPrice<br>PoolAddress  	|
