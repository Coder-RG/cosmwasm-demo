use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Contract expired (end_height {end_height:?}")]
    Expired { end_height: Option<u64> },

    #[error("Insufficient funds! Required: {funds:?}, Sent: {sent:?}")]
    InsufficientFunds { funds: u128, sent: Vec<Coin> },
}
