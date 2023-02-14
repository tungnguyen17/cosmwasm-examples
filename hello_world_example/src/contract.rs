use cosmwasm_std::{
  to_binary,
  Binary,
  Deps,
  DepsMut,
  Empty,
  Env,
  MessageInfo,
  Response,
  StdResult,
};
use crate::{
  msg::{
    AdminsListResp,
    InstantiateMsg,
    GreetResp,
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

#[allow(dead_code)]
pub fn execute(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  _msg: Empty,
) -> StdResult<Response> {
  unimplemented!()
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
