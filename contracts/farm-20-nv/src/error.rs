use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid reply ID")]
    InvalidReplyId {},

    #[error("Fail to bond")]
    FailBond {},

    #[error("Fail to unbond")]
    FailUnbond {},

    #[error("Overflow: {source}")]
    Overflow {
        source: OverflowError,
    },
}

impl From<OverflowError> for ContractError {
    fn from(msg: OverflowError) -> ContractError {
        match msg {
            overflow_error => ContractError::Overflow{ source: overflow_error },
        }
    }
}