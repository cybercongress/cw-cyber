use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("IncorrectInputData val: {val:?}")]
    IncorrectInputData {val: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Cannot migrate from different contract type: {previous_contract}")]
    CannotMigrate { previous_contract: String },
}
