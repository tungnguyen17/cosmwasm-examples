pub mod error;
pub mod exec;
pub mod inst;
pub mod msg;
pub mod query;
pub mod reply;
pub mod state;

use cosmwasm_std::{
  entry_point,
  Binary,
  Deps,
  DepsMut,
  Env,
  MessageInfo,
  Reply,
  Response,
  StdResult,
};
use cw_utils::{
  parse_reply_instantiate_data,
};
use crate::{
  error::{
    ContractError,
  },
  msg::{
    ExecuteMsg,
    InstantiateMsg,
    QueryMsg,
  },
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {

  inst::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  msg: ExecuteMsg
) -> Result<Response, ContractError> {

  let response = match msg {
    _ => Response::new(),
  };
  Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
  deps: Deps,
  env: Env,
  msg: QueryMsg,
) -> StdResult<Binary> {

  match msg {
    QueryMsg::QueryPublicKey { pubkey } => query::query_pubkey(deps, env, pubkey),
    QueryMsg::QueryPublicKeyRecovery { message_hash, signature } => query::query_pubkey_recovery(deps, env, message_hash, signature),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
  _deps: DepsMut,
  _env: Env,
  msg: Reply
) -> Result<Response, ContractError> {

  let reply = parse_reply_instantiate_data(msg.clone());

  if reply.is_err() {
    return Err(ContractError::InvalidReply);
  }

  let response = match msg.id {
    _ => Response::new(),
  };
  Ok(response)
}
