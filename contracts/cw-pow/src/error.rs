use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Wrong digest")]
    WrongDigest {},

    #[error("Wrong target")]
    WrongTarget {},

    #[error("Solution exist")]
    SolutionExist {},

    #[error("Supply for era exceeded")]
    SupplyExceeded {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
