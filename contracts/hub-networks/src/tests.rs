

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, Addr};
    use std::vec::Vec;
    use crate::state::{Entry, CONFIG, Config};
    use crate::msg::{ ListResponse, QueryMsg, InstantiateMsg, ExecuteMsg};
    use crate::contract::{query, execute, instantiate};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        //no owner specified in the instantiation message
        let msg = InstantiateMsg { owner: None };
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let state = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(
            state,
            Config {
                owner: Some(Addr::unchecked("creator".to_string())),
            }
        );
        //specifying an owner address in the instantiation message
        let msg = InstantiateMsg {
            owner: Some("specified_owner".to_string()),
        };

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let state = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(
            state,
            Config {
                owner: Some(Addr::unchecked("specified_owner".to_string())),
            }
        );
    }

    #[test]
    fn create_update_delete_entry() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg { owner: None };

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let msg = ExecuteMsg::CreateEntry {
            name: "tst".to_string(),
            protocol: "tst2".to_string(),
            chain_id: "cyber-1".to_string(),
            prefix: "cyb".to_string(),
            genesis_hash: "bostrom0x1".to_string(),
            unbonding_period: "42".to_string(),
            logo: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(),
            particle: Some("QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string()),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_create_entry"),
                attr("new_entry_id", "1")
            ]
        );

        let msg = ExecuteMsg::CreateEntry {
            name: "tst2".to_string(),
            protocol: "tst2".to_string(),
            chain_id: "cyber-1".to_string(),
            prefix: "cyb".to_string(),
            genesis_hash: "bostrom0x1".to_string(),
            unbonding_period: "42".to_string(),
            logo: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
            particle: Some("QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string()), 
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_create_entry"),
                attr("new_entry_id", "2")
            ]
        );

        // Query the list of entries
        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::GetEntries {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let list: ListResponse = from_binary(&res).unwrap();
        assert_eq!(
            Vec::from([
                Entry {
                    id: 1,
                    name: "tst".to_string(),
                    protocol: "tst2".to_string(),
                    chain_id: "cyber-1".to_string(),
                    prefix: "cyb".to_string(),
                    genesis_hash: "bostrom0x1".to_string(),
                    unbonding_period: "42".to_string(),
                    logo: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
                    particle: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
                },
                Entry {
                    id: 2,
                    name: "tst2".to_string(),
                    protocol: "tst2".to_string(),
                    chain_id: "cyber-1".to_string(),
                    prefix: "cyb".to_string(),
                    genesis_hash: "bostrom0x1".to_string(),
                    unbonding_period: "42".to_string(),
                    logo: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
                    particle: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
                }
            ]),
            list.entries
        );

        // Update entry
        let message = ExecuteMsg::UpdateEntry {
            id: 1,
            name: Some("tstu".to_string()),
            protocol: Some("tst2".to_string()),
            chain_id: Some("cyber-1".to_string()),
            prefix: Some("cyb".to_string()),
            genesis_hash: Some("bostrom0x1".to_string()),
            unbonding_period: Some("42".to_string()),
            logo: Some("QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string()), 
            particle: Some("QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string()), 
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), message).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_update_entry"),
                attr("updated_entry_id", "1")
            ]
        );

        

        // Query the list of entries
        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::GetEntries {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let list: ListResponse = from_binary(&res).unwrap();
        assert_eq!(
            Vec::from([
                Entry {
                    id: 1,
                    name: "tstu".to_string(),
                    protocol: "tst2".to_string(),
                    chain_id: "cyber-1".to_string(),
                    prefix: "cyb".to_string(),
                    genesis_hash: "bostrom0x1".to_string(),
                    unbonding_period: "42".to_string(),
                    logo: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
                    particle: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
                },
                Entry {
                    id: 2,
                    name: "tst2".to_string(),
                    protocol: "tst2".to_string(),
                    chain_id: "cyber-1".to_string(),
                    prefix: "cyb".to_string(),
                    genesis_hash: "bostrom0x1".to_string(),
                    unbonding_period: "42".to_string(),
                    logo: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
                    particle: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
                }
            ]),
            list.entries
        );

        //Delete Entry
        let message = ExecuteMsg::DeleteEntry { id: 1 };

        let res = execute(deps.as_mut(), env.clone(), info, message).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_delete_entry"),
                attr("deleted_entry_id", "1")
            ]
        );
        // Query the list of entries
        let res = query(
            deps.as_ref(),
            env,
            QueryMsg::GetEntries {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let list: ListResponse = from_binary(&res).unwrap();
        assert_eq!(
            Vec::from([Entry {
                id: 2,
                name: "tst2".to_string(),
                protocol: "tst2".to_string(),
                chain_id: "cyber-1".to_string(),
                prefix: "cyb".to_string(),
                genesis_hash: "bostrom0x1".to_string(),
                unbonding_period: "42".to_string(),
                logo: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
                particle: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(), 
            }]),
            list.entries
        );
    }
}
