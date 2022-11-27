use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Expired")]
    Expired {},

    #[error("Funding ended")]
    FundingEnded {},

    #[error("Funding period")]
    FundingPeriod {},

    #[error("Token locked")]
    TokenLocked {},

    #[error("Funds claimed")]
    FundsClaimed {},

    #[error("Vesting Period")]
    VestingPeriod {},

    #[error("Cannot migrate from different contract type: {previous_contract}")]
    CannotMigrate { previous_contract: String },

    #[error("Cannot migrate from unsupported version: {previous_version}")]
    CannotMigrateVersion { previous_version: String },

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
