use cosmwasm_schema::{
  write_api,
};
use hello_world::{
  msg::{
    ExecuteMsg,
    InstantiateMsg,
    QueryMsg,
  },
};

fn main() {
  write_api! {
    instantiate: InstantiateMsg,
    execute: ExecuteMsg,
    query: QueryMsg
  }
}
