use cosmwasm_std::{
  Addr,
};
use serde::{
  Deserialize,
  Serialize
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
  pub admins: Vec<String>,
  pub donation_denom: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {
  AddMembers { admins: Vec<String> },
  Donate {},
  Leave {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
  AdminsList {},
  Greet {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AdminsListResp {
  pub admins: Vec<Addr>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GreetResp {
  pub message: String,
}
