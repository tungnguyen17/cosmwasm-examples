use cosmwasm_std::{
  Addr,
  StdError,
};
use thiserror::{
  Error
};

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
  #[error("Invalid message")]
  InvalidMsg,
  #[error("Invalid reply")]
  InvalidReply,
  #[error("{0}")]
  StdError(#[from] StdError),
  #[error("{sender} is not authorized")]
  Unauthorized { sender: Addr },
}
