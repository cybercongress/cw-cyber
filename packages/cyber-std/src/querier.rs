use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::route::CyberRoute;
use crate::query::{
    CyberQuery, CyberQueryWrapper,
    RankValueResponse, CidsCountResponse, LinksCountResponse,
    JobResponse, JobStatsResponse, LowestFeeResponse,
    RoutesResponse, RouteResponse, RoutedEnergyResponse,
    PriceResponse, LoadResponse, DesirableBandwidthResponse, AccountBandwidthResponse
};

pub struct CyberQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

impl<'a> CyberQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper) -> Self {
        CyberQuerier { querier }
    }

    // pub fn query_rank_value_by_id<T: Into<u64>>(
    //     &self,
    //     cid_number: T,
    // ) -> StdResult<RankValueResponse> {
    //     let request = CyberQueryWrapper {
    //         route: CyberRoute::Rank,
    //         query_data: CyberQuery::RankValueById {
    //             cid_number: cid_number.into(),
    //         },
    //     };
    //
    //     let res: RankValueResponse = self.querier.custom_query(&request.into())?;
    //     Ok(res)
    // }

    pub fn query_rank_value_by_cid<T: Into<String>>(
        &self,
        cid: T,
    ) -> StdResult<RankValueResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Rank,
            query_data: CyberQuery::GetRankValueByCid {
                cid: cid.into(),
            },
        };

        let res: RankValueResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_cids_count(&self) -> StdResult<CidsCountResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Graph,
            query_data: CyberQuery::GetCidsCount {},
        };
        let res: CidsCountResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_links_count(&self) -> StdResult<LinksCountResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Graph,
            query_data: CyberQuery::GetLinksCount {},
        };
        let res: LinksCountResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_job<T: Into<String>>(
        &self,
        creator: T,
        contract: T,
        label: T,
    ) -> StdResult<JobResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Cron,
            query_data: CyberQuery::GetJob {
                creator: creator.into(),
                contract: contract.into(),
                label: label.into(),
            },
        };
        let res: JobResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_job_stats<T: Into<String>>(
        &self,
        creator: T,
        contract: T,
        label: T,
    ) -> StdResult<JobStatsResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Cron,
            query_data: CyberQuery::GetJobStats {
                creator: creator.into(),
                contract: contract.into(),
                label: label.into(),
            },
        };
        let res: JobStatsResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_lowest_fee(&self) -> StdResult<LowestFeeResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Cron,
            query_data: CyberQuery::GetLowestFee {},
        };
        let res: LowestFeeResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_source_routes<T: Into<String>>(
        &self,
        source: T,
    ) -> StdResult<RoutesResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Energy,
            query_data: CyberQuery::GetSourceRoutes {
                source: source.into(),
            },
        };
        let res: RoutesResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_source_routed_energy<T: Into<String>>(
        &self,
        source: T,
    ) -> StdResult<RoutedEnergyResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Energy,
            query_data: CyberQuery::GetSourceRoutedEnergy {
                source: source.into(),
            },
        };
        let res: RoutedEnergyResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_destination_routed_energy<T: Into<String>>(
        &self,
        destination: T,
    ) -> StdResult<RoutedEnergyResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Energy,
            query_data: CyberQuery::GetDestinationRoutedEnergy {
                destination: destination.into(),
            },
        };
        let res: RoutedEnergyResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_route<T: Into<String>>(
        &self,
        source: T,
        destination: T,
    ) -> StdResult<RouteResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Energy,
            query_data: CyberQuery::GetRoute {
                source: source.into(),
                destination: destination.into(),
            },
        };
        let res: RouteResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_price(&self) -> StdResult<PriceResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Bandwidth,
            query_data: CyberQuery::GetPrice {},
        };
        let res: PriceResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_load(&self) -> StdResult<LoadResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Bandwidth,
            query_data: CyberQuery::GetLoad {},
        };
        let res: LoadResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_desirable_bandwidth(&self) -> StdResult<DesirableBandwidthResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Bandwidth,
            query_data: CyberQuery::GetDesirableBandwidth {},
        };
        let res: DesirableBandwidthResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    pub fn query_account_bandwidth<T: Into<String>>(
        &self,
        address: T,
    ) -> StdResult<AccountBandwidthResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Bandwidth,
            query_data: CyberQuery::GetAccountBandwidth {
                address: address.into(),
            },
        };
        let res: AccountBandwidthResponse = self.querier.custom_query(&request.into())?;
        Ok(res)
    }
}
