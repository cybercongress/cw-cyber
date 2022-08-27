# cyber~Farms

Cyber farms allow citizens to stake their cw20 and native tokens and get rewards for staking with cw20 and native tokens. Citizens get a liquid position in a farm with staking.

# Cyber farms:
1. Farm #1 (farm-20-20)
    - stake cw20 token
    - get cw20 token reward
2. Farm #2 (farm-20-nv)
    - stake cw20 token
    - get native token reward
3. Farm #3 (farm-nv-20)
    - stake native token
    - get cw20 token reward
4. Farm #4 (farm-nv-nv)
    - stake native token
    - get native token reward

## Liquid Staking Token
When cw20 or native tokens are staked to farm then staker gets their position as cw20 token (liquid staking). All farms stake position tokens are minted with symbol `CFLST` (CyberFarm Liquid Staking Token) and name `CyberFarm V1 - POOL_NAME`.

## Rewards
- Farm rewards are configurable with a given distribution schedule.
- Farm's operator initializes farm with given time schedules that consist of blocks periods and tokens allocation for given periods.
- Farm's operator may extend rewards later by adding new periods and tokens allocations to the existing distribution schedule.

Example:
```
[[0, 10000, 5000], [10000, 20000, 5000], [20000, 30000, 5000], [30000, 40000, 5000], [50000, 50000, 5000]]
```
Will allocate 5000 of the given token to stakers as rewards for each 10000 block period starting from block 0 and ending at block 50000.

## Deploy
```
{
    // account that manage farm
    "distribution_account": "bostrom1nfmvw8x37w00p3geuu8lrt3vt5kadxa5xd9us7",
    
    // reward_denom OR reward_token
    "reward_denom": "boot",
    "reward_token": "bostrom1sms4u3vra5wem5dufl7wwttzyrcgfe529u9rp2rqdst60skllzgsuthkv4",
    
    // staking_denom OR staking_token
    "staking_denom": "hydrogen",
    "staking_token": "bostrom1sms4u3vra5wem5dufl7wwttzyrcgfe529u9rp2rqdst60skllzgsuthkv4",
    
    
    "distribution_schedule": [[1359700,1360000,"1000000000"],[1360000, 1361000,"1000000000"],[1361000,1362000,"1000000000"],[1362000,1363000,"1000000000"],[1363000,1364000,"1000000000"]],
    
    // code_id of deployed CW20 contract that will be issued for liquid position tokens
    "token_code_id": 1,
    
    // name of the pool that will be used as part of the name of liquid position token
    "pool_name": "POOL-10-LP",
}
```