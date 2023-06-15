

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
            active: "active".to_string(),
            source_chain_id: "bostrom-1".to_string(),
            destination_chain_id: "evmos_9001-2".to_string(),
            source_channel_id: "channel-256".to_string(),
            destination_channel_id: "channel-1".to_string(),
            explorer_url: "https://explorer.com/{hash}".to_string(),
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
            active: "active".to_string(),
            source_chain_id: "evmos_9001-2".to_string(),
            destination_chain_id: "cosmos-2".to_string(),
            source_channel_id: "channel-2".to_string(),
            destination_channel_id: "channel-2".to_string(),
            explorer_url: "https://explorer.com/{hash}".to_string(),
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
                    active: "active".to_string(),
                    source_chain_id: "bostrom-1".to_string(),
                    destination_chain_id: "evmos_9001-2".to_string(),
                    source_channel_id: "channel-256".to_string(),
                    destination_channel_id: "channel-1".to_string(),
                    explorer_url: "https://explorer.com/{hash}".to_string(),
                    particle: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(),
                },
                Entry {
                    id: 2,
                    active: "active".to_string(),
                    source_chain_id: "evmos_9001-2".to_string(),
                    destination_chain_id: "cosmos-2".to_string(),
                    source_channel_id: "channel-2".to_string(),
                    destination_channel_id: "channel-2".to_string(),
                    explorer_url: "https://explorer.com/{hash}".to_string(),
                    particle: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(),
                }
            ]),
            list.entries
        );

        // Update entry
        let message = ExecuteMsg::UpdateEntry {
            id: 1,
            active: "active".to_string(),
            source_chain_id: Some("evmos_9001-2".to_string()),
            destination_chain_id: Some("cosmos-1".to_string()),
            source_channel_id: Some("channel-256".to_string()),
            destination_channel_id: Some("channel-1".to_string()),
            explorer_url: Some("https://explorer.com/{hash}".to_string()),
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
                    active: "active".to_string(),
                    source_chain_id: "evmos_9001-2".to_string(),
                    destination_chain_id: "cosmos-1".to_string(),
                    source_channel_id: "channel-256".to_string(),
                    destination_channel_id: "channel-1".to_string(),
                    explorer_url: "https://explorer.com/{hash}".to_string(),
                    particle: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(),
                },
                Entry {
                    id: 2,
                    active: "active".to_string(),
                    source_chain_id: "evmos_9001-2".to_string(),
                    destination_chain_id: "cosmos-2".to_string(),
                    source_channel_id: "channel-2".to_string(),
                    destination_channel_id: "channel-2".to_string(),
                    explorer_url: "https://explorer.com/{hash}".to_string(),
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
                active: "active".to_string(),
                source_chain_id: "evmos_9001-2".to_string(),
                destination_chain_id: "cosmos-2".to_string(),
                source_channel_id: "channel-2".to_string(),
                destination_channel_id: "channel-2".to_string(),
                explorer_url: "https://explorer.com/{hash}".to_string(),
                particle: "QmYpTB36duejmy1szbdL1D2EzC5fgRL4dyhSFsHkMYPtny".to_string(),
            }]),
            list.entries
        );
    }
}
