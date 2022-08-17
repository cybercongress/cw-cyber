

#[cfg(test)]
mod tests {
    // use super::*;
    // use cosmwasm_std::{Uint64};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, Addr};
    use std::vec::Vec;
    use crate::state::{Entry, CONFIG, Config};
    use crate::msg::{ ListResponse, QueryMsg, InstantiateMsg, ExecuteMsg};
    // use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
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
                owner: Addr::unchecked("creator".to_string()),
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
                owner: Addr::unchecked("specified_owner".to_string()),
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

        let msg = ExecuteMsg::NewEntry {
            channel_id: "TST".to_string(),
            network_source: "TESTCOIN".to_string(),
            network_destination: "TESTCOIN".to_string(),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_create_new_item"),
                attr("new_entry_id", "1")
            ]
        );

        let msg = ExecuteMsg::NewEntry {
            channel_id: "TST2".to_string(),
            network_source: "TESTCOIN2".to_string(),
            network_destination: "TESTCOIN2".to_string(),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_create_new_item"),
                attr("new_entry_id", "2")
            ]
        );

        // Query the list of entries
        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::GetTokens {
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
                    channel_id: "TST".to_string(),
                    network_source: "TESTCOIN".to_string(),
                    network_destination: "TESTCOIN".to_string(),
                },
                Entry {
                    id: 2,
                    channel_id: "TST2".to_string(),
                    network_source: "TESTCOIN2".to_string(),
                    network_destination: "TESTCOIN2".to_string(),
                }
            ]),
            list.entries
        );

        // Update entry
        let message = ExecuteMsg::UpdateEntry {
            id: 1,
            channel_id: Some("TSTUPDATE".to_string()),
            network_source: Some("TESTCOINUPDATE".to_string()),
            network_destination: Some("TESTCOINUPDATE".to_string()),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), message).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "execute_update_item"),
                attr("updated_entry_id", "1")
            ]
        );

        

        // Query the list of entries
        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::GetTokens {
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
                    channel_id: "TSTUPDATE".to_string(),
                    network_source: "TESTCOINUPDATE".to_string(),
                    network_destination: "TESTCOINUPDATE".to_string(),
                },
                Entry {
                    id: 2,
                    channel_id: "TST2".to_string(),
                    network_source: "TESTCOIN2".to_string(),
                    network_destination: "TESTCOIN2".to_string(),
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
            QueryMsg::GetTokens {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let list: ListResponse = from_binary(&res).unwrap();
        assert_eq!(
            Vec::from([Entry {
                id: 2,
                channel_id: "TST2".to_string(),
                network_source: "TESTCOIN2".to_string(),
                network_destination: "TESTCOIN2".to_string(),
            }]),
            list.entries
        );
    }
}
