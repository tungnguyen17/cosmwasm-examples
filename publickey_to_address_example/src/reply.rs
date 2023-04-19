use cosmwasm_std::{
  DepsMut,
  Env,
  Response,
};
use crate::{
  error::{
    ContractError,
  },
};

pub fn reply_foo(
  _deps: DepsMut,
  _env: Env,
) -> Result<Response, ContractError> {

  Ok(Response::new())
}
