use cosmwasm_schema::{
  cw_serde,
  QueryResponses,
};

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
  ExecFoo {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
  #[returns(QueryPublicKeyResp)]
  QueryPublicKey {
    pubkey: Vec<u8>,
  },
  #[returns(QueryPublicKeyRecoveryResp)]
  QueryPublicKeyRecovery {
    message_hash: Vec<u8>,
    signature: Vec<u8>,
  },
}

#[cw_serde]
pub struct QueryPublicKeyResp {
  pub uncompressed_pubkey_hex: String,
  pub uncompressed_pubkey_x_hex: String,
  pub uncompressed_pubkey_y_hex: String,
  pub pubkey_hex: String,
  pub raw_address_hex: String,
  pub canonical_address: String,
  pub humanized_address: String,
  pub bech32_address: String,
}

#[cw_serde]
pub struct QueryPublicKeyRecoveryResp {
  pub uncompressed_pubkey_hex: String,
  pub uncompressed_pubkey_x_hex: String,
  pub uncompressed_pubkey_y_hex: String,
  pub pubkey_hex: String,
  pub raw_address_hex: String,
  pub canonical_address: String,
  pub humanized_address: String,
  pub bech32_address: String,
}

pub enum ReplyId {
}
