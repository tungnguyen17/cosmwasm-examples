use cosmwasm_std::{
  DepsMut,
  Env,
  MessageInfo,
  Response,
};
use crate::{
  error::{
    ContractError,
  },
};

pub fn execute_foo(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
) -> Result<Response, ContractError> {

  Ok(Response::new())
}
