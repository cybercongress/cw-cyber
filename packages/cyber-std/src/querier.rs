use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::route::CyberRoute;
use crate::query::{CyberQuery, CyberQueryWrapper, RankValueResponse, CidsCountResponse, LinksCountResponse};

/// This is a helper wrapper to easily use our custom queries
pub struct CyberQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

impl<'a> CyberQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper) -> Self {
        CyberQuerier { querier }
    }

    pub fn query_rank_value_by_id<T: Into<u64>>(
        &self,
        cid_number: T,
    ) -> StdResult<RankValueResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Rank,
            query_data: CyberQuery::RankValueById {
                cid_number: cid_number.into(),
            },
        };

        let res: RankValueResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_rank_value_by_cid<T: Into<String>>(
        &self,
        cid: T,
    ) -> StdResult<RankValueResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Rank,
            query_data: CyberQuery::RankValueByCid {
                cid: cid.into(),
            },
        };

        let res: RankValueResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_cids_count(&self) -> StdResult<CidsCountResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Graph,
            query_data: CyberQuery::CidsCount {},
        };
        let res: CidsCountResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_links_count(&self) -> StdResult<LinksCountResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Graph,
            query_data: CyberQuery::LinksCount {},
        };
        let res: LinksCountResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }
}
