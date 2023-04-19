use bech32::{
  ToBase32,
};
use cosmwasm_std::{
  to_binary,
  Binary,
  CanonicalAddr,
  Deps,
  Env,
  StdResult,
};
use hex;
use ripemd::{
  Ripemd160,
};
use sha2::{
  Digest,
  Sha256,
};
use crate::{
  msg::{
    QueryPublicKeyResp,
    QueryPublicKeyRecoveryResp,
  },
};

pub fn query_pubkey(
  deps: Deps,
  _env: Env,
  pubkey: Vec<u8>,
) -> StdResult<Binary> {

  let pubkey_x = &pubkey[1..33];
  let pubkey_y = &pubkey[33..65];
  let prefix = if pubkey_y[31] % 2 == 0 { 2 } else { 3 };
  let mut compressed_pubkey = Vec::<u8>::new();
  compressed_pubkey.extend_from_slice(&[prefix]);
  compressed_pubkey.extend_from_slice(pubkey_x);

  let compressed_pubkey_hashed_1 = &Sha256::digest(&compressed_pubkey);
  let compressed_pubkey_hashed_2 = &Ripemd160::digest(&compressed_pubkey_hashed_1);
  let canonical_address = CanonicalAddr::from(&compressed_pubkey_hashed_2[..]);
  let humanized_address = deps.api.addr_humanize(&canonical_address).unwrap();
  let bech32_address = bech32::encode("wasm", &compressed_pubkey_hashed_2.to_base32(), bech32::Variant::Bech32).unwrap();

  let resp = QueryPublicKeyResp {
    pubkey_hex: hex::encode(&pubkey[..]),
    pubkey_x_hex: hex::encode(pubkey_x),
    pubkey_y_hex: hex::encode(pubkey_y),
    compressed_pubkey_hex: hex::encode(compressed_pubkey),
    raw_address_hex: hex::encode(compressed_pubkey_hashed_2),
    canonical_address: canonical_address.to_string(),
    humanized_address: humanized_address.to_string(),
    bech32_address,
  };
  to_binary(&resp)
}

pub fn query_pubkey_recovery(
  deps: Deps,
  _env: Env,
  message_hash: Vec<u8>,
  signature: Vec<u8>,
) -> StdResult<Binary> {

  let pubkey = deps.api
    .secp256k1_recover_pubkey(&message_hash, &signature, 0u8).unwrap();
  let pubkey_x = &pubkey[1..33];
  let pubkey_y = &pubkey[33..65];
  let prefix = if pubkey_y[31] % 2 == 0 { 2 } else { 3 };
  let mut compressed_pubkey = Vec::<u8>::new();
  compressed_pubkey.extend_from_slice(&[prefix]);
  compressed_pubkey.extend_from_slice(pubkey_x);

  let compressed_pubkey_hashed_1 = &Sha256::digest(&compressed_pubkey);
  let compressed_pubkey_hashed_2 = &Ripemd160::digest(&compressed_pubkey_hashed_1);
  let canonical_address = CanonicalAddr::from(&compressed_pubkey_hashed_2[..]);
  let humanized_address = deps.api.addr_humanize(&canonical_address).unwrap();
  let bech32_address = bech32::encode("wasm", &compressed_pubkey_hashed_2.to_base32(), bech32::Variant::Bech32).unwrap();


  let resp = QueryPublicKeyRecoveryResp {
    pubkey_hex: hex::encode(&pubkey[..]),
    pubkey_x_hex: hex::encode(pubkey_x),
    pubkey_y_hex: hex::encode(pubkey_y),
    compressed_pubkey_hex: hex::encode(compressed_pubkey),
    raw_address_hex: hex::encode(compressed_pubkey_hashed_2),
    canonical_address: canonical_address.to_string(),
    humanized_address: humanized_address.to_string(),
    bech32_address,
  };
  to_binary(&resp)
}
