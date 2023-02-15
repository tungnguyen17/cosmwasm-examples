use cosmwasm_std::{
  to_binary,
  Binary,
  Deps,
  DepsMut,
  Env,
  MessageInfo,
  Response,
  StdResult,
};
use crate::{
  msg::{
    AdminsListResp,
    ExecuteMsg,
    GreetResp,
    InstantiateMsg,
    QueryMsg,
  },
  state::{
    ADMINS,
  },
};

pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  msg: InstantiateMsg,
) -> StdResult<Response> {
  let admins: StdResult<Vec<_>> = msg
    .admins
    .into_iter()
    .map(|addr| deps.api.addr_validate(&addr))
    .collect();
  ADMINS.save(deps.storage, &admins?)?;

  Ok(Response::new())
}

pub fn execute(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> StdResult<Response> {
  use ExecuteMsg::*;

  match msg {
    AddMembers { admins } => exec::add_members(deps, info, admins),
    Leave {} => exec::leave(deps, info),
  }
}

pub fn query(
  deps: Deps,
  _env: Env,
  msg: QueryMsg,
) -> StdResult<Binary> {
  use QueryMsg::*;

  match msg {
    AdminsList {} => to_binary(&query::admins_list(deps)?),
    Greet {} => to_binary(&query::greet()?),
  }
}

mod exec {
  use super::*;
  use cosmwasm_std::{
    StdError,
  };

  pub fn add_members(
    deps: DepsMut,
    info: MessageInfo,
    admins: Vec<String>,
  ) -> StdResult<Response> {
    let mut curr_admins = ADMINS.load(deps.storage)?;
    if !curr_admins.contains(&info.sender) {
      return Err(StdError::generic_err("Unauthorised access"));
    }

    let admins: StdResult<Vec<_>> = admins.into_iter()
      .map(|addr| deps.api.addr_validate(&addr))
      .collect();

    curr_admins.append(&mut admins?);
    ADMINS.save(deps.storage, &curr_admins)?;

    Ok(Response::new())
  }

  pub fn leave(
    deps: DepsMut,
    info: MessageInfo,
  ) -> StdResult<Response> {
    ADMINS.update(deps.storage, move |admins| -> StdResult<_> {
      let admins = admins.into_iter()
        .filter(|admin| *admin != info.sender)
        .collect();
      Ok(admins)
    })?;

    Ok(Response::new())
  }
}

mod query {
  use super::*;

  pub fn admins_list(deps: Deps) -> StdResult<AdminsListResp> {
    let admins = ADMINS.load(deps.storage)?;
    let resp = AdminsListResp { admins };
    Ok(resp)
  }

  pub fn greet() -> StdResult<GreetResp> {
    let resp = GreetResp {
      message: "Hello World".to_owned(),
    };

    Ok(resp)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cosmwasm_std::{
    Addr,
  };
  use cw_multi_test::{
    App,
    ContractWrapper,
    Executor,
  };

  #[test]
  fn greet_query() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app.instantiate_contract(
      code_id,
      Addr::unchecked("owner"),
      &InstantiateMsg {
        admins: Vec::new(),
      },
      &[],
      "Contract",
      None,
    ).unwrap();

    let resp: GreetResp = app.wrap()
      .query_wasm_smart(addr, &QueryMsg::Greet {})
      .unwrap();

    assert_eq!(
      resp,
      GreetResp {
        message: "Hello World".to_owned()
      }
    );
  }

  #[test]
  fn instantiation() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app.instantiate_contract(
      code_id,
      Addr::unchecked("owner"),
      &InstantiateMsg { admins: vec![] },
      &[],
      "Contract",
      None,
    ).unwrap();

    let resp: AdminsListResp = app.wrap()
      .query_wasm_smart(addr, &QueryMsg::AdminsList {})
      .unwrap();

    assert_eq!(resp, AdminsListResp { admins: vec![] });

    let addr = app.instantiate_contract(
      code_id,
      Addr::unchecked("owner"),
      &InstantiateMsg {
          admins: vec!["admin1".to_owned(), "admin2".to_owned()],
      },
      &[],
      "Contract 2",
      None,
    ).unwrap();

    let resp: AdminsListResp = app.wrap()
      .query_wasm_smart(addr, &QueryMsg::AdminsList {})
      .unwrap();

    assert_eq!(
      resp,
      AdminsListResp {
        admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
      }
    );
  }
}
