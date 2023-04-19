use std::{
  collections::{
    HashMap,
  },
};
use cosmwasm_std::{
  Addr,
};
use cw_storage_plus::{
  Item,
};

pub const ADDR: Item<Addr> = Item::new("address");
pub const LIST: Item<Vec<u128>> = Item::new("numbers");
pub const MAPPING: Item<HashMap<u128, u128>> = Item::new("mapping");
pub const STR: Item<String> = Item::new("str");
