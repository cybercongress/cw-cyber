use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::route::CyberRoute;
use crate::query::{
    CyberQuery, CyberQueryWrapper,
    ParticleRankResponse, ParticlesAmountResponse, CyberlinksAmountResponse,
    ThoughtResponse, ThoughtStatsResponse, LowestFeeResponse,
    RoutesResponse, RouteResponse, RoutedEnergyResponse,
    BandwidthPriceResponse, BandwidthLoadResponse, BandwidthTotalResponse, NeuronBandwidthResponse
};

pub struct CyberQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

impl<'a> CyberQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper) -> Self {
        CyberQuerier { querier }
    }

    pub fn query_particle_rank<T: Into<String>>(
        &self,
        particle: T,
    ) -> StdResult<ParticleRankResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Rank,
            query_data: CyberQuery::ParticleRank {
                particle: particle.into(),
            },
        };

        let res: ParticleRankResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_particles_amount(&self) -> StdResult<ParticlesAmountResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Graph,
            query_data: CyberQuery::ParticlesAmount {},
        };
        let res: ParticlesAmountResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_cyberlinks_amount(&self) -> StdResult<CyberlinksAmountResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Graph,
            query_data: CyberQuery::CyberlinksAmount {},
        };
        let res: CyberlinksAmountResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_thought<T: Into<String>>(
        &self,
        program: T,
        name: T,
    ) -> StdResult<ThoughtResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Dmn,
            query_data: CyberQuery::Thought {
                program: program.into(),
                name: name.into(),
            },
        };
        let res: ThoughtResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_thought_stats<T: Into<String>>(
        &self,
        program: T,
        name: T,
    ) -> StdResult<ThoughtStatsResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Dmn,
            query_data: CyberQuery::ThoughtStats {
                program: program.into(),
                name: name.into(),
            },
        };
        let res: ThoughtStatsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_lowest_fee(&self) -> StdResult<LowestFeeResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Dmn,
            query_data: CyberQuery::LowestFee {},
        };
        let res: LowestFeeResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_source_routes<T: Into<String>>(
        &self,
        source: T,
    ) -> StdResult<RoutesResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Grid,
            query_data: CyberQuery::SourceRoutes {
                source: source.into(),
            },
        };
        let res: RoutesResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_source_routed_energy<T: Into<String>>(
        &self,
        source: T,
    ) -> StdResult<RoutedEnergyResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Grid,
            query_data: CyberQuery::SourceRoutedEnergy {
                source: source.into(),
            },
        };
        let res: RoutedEnergyResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_destination_routed_energy<T: Into<String>>(
        &self,
        destination: T,
    ) -> StdResult<RoutedEnergyResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Grid,
            query_data: CyberQuery::DestinationRoutedEnergy {
                destination: destination.into(),
            },
        };
        let res: RoutedEnergyResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_route<T: Into<String>>(
        &self,
        source: T,
        destination: T,
    ) -> StdResult<RouteResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Grid,
            query_data: CyberQuery::Route {
                source: source.into(),
                destination: destination.into(),
            },
        };
        let res: RouteResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_bandwidth_price(&self) -> StdResult<BandwidthPriceResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Bandwidth,
            query_data: CyberQuery::BandwidthPrice {},
        };
        let res: BandwidthPriceResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_bandwidth_load(&self) -> StdResult<BandwidthLoadResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Bandwidth,
            query_data: CyberQuery::BandwidthLoad {},
        };
        let res: BandwidthLoadResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_bandwidth_total(&self) -> StdResult<BandwidthTotalResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Bandwidth,
            query_data: CyberQuery::BandwidthTotal {},
        };
        let res: BandwidthTotalResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_neuron_bandwidth<T: Into<String>>(
        &self,
        address: T,
    ) -> StdResult<NeuronBandwidthResponse> {
        let request = CyberQueryWrapper {
            route: CyberRoute::Bandwidth,
            query_data: CyberQuery::NeuronBandwidth {
                neuron: address.into(),
            },
        };
        let res: NeuronBandwidthResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}
