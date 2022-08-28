# Cyber Bindings for CosmWasm

![Crates.io](https://img.shields.io/crates/v/cyber-std)
![Crates.io](https://img.shields.io/crates/d/cyber-std)

This crate provides Cyber-specific bindings to enable your CosmWasm smart contracts to interact with the Cyber blockchain by exposing messages and queriers that can be emitted and used from within your contract.

## Programs
| Program                                              	| Status                      	| Version 	| Description                                                                                          	| bostrom 	| space-pussy-1 	|
|------------------------------------------------------	|-----------------------------	|---------	|------------------------------------------------------------------------------------------------------	|---------	|---------------	|
| cw1-subkeys                                          	| - Ready to production       	|         	| - Proxy contract<br>- Cyber's msgs support<br>- DMN support                                          	|         	|               	|
| cw1-whitelist                                        	| - Ready to produciton       	|         	| - Proxy contract<br>- Cyber's msgs support<br>- DMN support                                          	|         	|               	|
| cw3-fixed-multisig                                   	| - Ready to production       	|         	| - Static multisig/clan<br>- Cyber's msgs support<br>- DMN support                                    	|         	|               	|
| cw3-flex-multisig                                    	| - Ready to productioin      	|         	| - Dynamic multisig/clan<br>- Cyber's msgs support<br>- DMN support                                   	|         	|               	|
| farm-nv-nv<br>farm-nv-20<br>farm-20-nv<br>farm-20-20 	| - Ready to testnet          	|         	| - Stake native/cw20 token<br>- Farm native/cw20 token<br>- cw20 LP<br>- Rewards program updates<br>  	|         	|               	|
| neuron-booster                                       	| - R&D<br>- Ready to testnet 	|         	|                                                                                                      	|         	|               	|
| particle-booster                                     	| - R&D                       	|         	|                                                                                                      	|         	|               	|
| cyberlink-booster                                    	| - R&D                       	|         	|                                                                                                      	|         	|               	|
| prediction-markets                                   	| - R&D                       	|         	| - Conditionals tokens                                                                                	|         	|               	|
| cw721-pow                                            	| - Development               	|         	| - Mining of NFT<br>- Keccak256                                                                       	|         	|               	|
| cw20-pow                                             	| - Development               	|         	| - Mining of cw20 token<br>- Keccak256                                                                	|         	|               	|
| native-pow                                           	| - Development               	|         	| - Mining of native token<br>- Keccak256                                                              	|         	|               	|
| reflect<br>ibc-reflect<br>ibc-reflect-send           	| - Ready to production       	|         	|                                                                                                      	|         	|               	|
| ica-controller<br>ica-host                           	| - Ready to production       	|         	|                                                                                                      	|         	|               	|
| cyber-swap                                           	| - Development               	|         	|                                                                                                      	|         	|               	|
| cw-passport                                          	| - Ready to production       	|         	|                                                                                                      	|         	|               	|
| cw-cybergift                                         	| - Ready to production       	|         	|                                                                                                      	|         	|               	|
| cw-subgraph                                          	| - Ready to production       	|         	|                                                                                                      	|         	|               	|
| cw-zk-nft                                            	| - Research                  	|         	|                                                                                                      	|         	|               	|

## Packages

Currently, the cyber-std Cyber bindings include:

| Module    	| Execute                                                                                                                                                                                          	| Query                                                                  	|
|-----------	|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------	|------------------------------------------------------------------------	|
| Graph     	| MsgCyberlink                                                                                                                                                                                     	| CyberlinkExist<br>ParticleExist<br>ParticlesAmount<br>CyberlinksAmount 	|
| Rank      	|                                                                                                                                                                                                  	| ParticleRank                                                           	|
| Bandwidth 	|                                                                                                                                                                                                  	| BandwidthPrice<br>BandwidthLoad<br>BandwidthTotal<br>NeuronBandwidth   	|
| Resources 	| MsgInvestmint                                                                                                                                                                                    	|                                                                        	|
| Grid      	| MsgCreateRoute<br>MsgEditRoute<br>MsgEditRouteName<br>MsgDeleteRoute                                                                                                                             	| SourceRoutes<br>SourceRoutedEnergy<br>DestinationRoutedEnergy<br>Route 	|
| DMN       	| MsgCreateThought<br>MsgForgetThought<br>MsgChangeThoughtInput<br>MsgChangeThoughtPeriod<br>MsgChangeThoughtBlock<br>MsgChangeThoughtGasPrice<br>MsgChangeThoughtParticle<br>MsgChangeThoughtName 	| Thought<br>ThoughtStats<br>ThoughtLowestFee                            	|
| Liquidity 	| MsgCreatePool<br>MsgDepositWithinBath<br>MsgWithdrawWithinBath<br>MsgSwapWithinBath                                                                                                              	| PoolParams<br>PoolLiquidity<br>PoolSupply<br>PoolPrice<br>PoolAddress  	|

PS: There is cyber-std-test with tooling for writing test for multiple contracts
