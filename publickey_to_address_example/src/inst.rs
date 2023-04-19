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
  msg::{
    InstantiateMsg,
  },
};

pub fn instantiate(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

  Ok(Response::new())
}
