use serde::{
  Deserialize,
  Serialize
};

#[derive(Serialize, Deserialize)]
pub enum QueryMsg {
  Greet {},
}

#[derive(Serialize, Deserialize)]
pub struct GreetResp {
  pub message: String,
}
