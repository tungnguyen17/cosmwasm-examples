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
  uncompressed_pubkey: Vec<u8>,
) -> StdResult<Binary> {

  let uncompressed_pubkey_x = &uncompressed_pubkey[1..33];
  let uncompressed_pubkey_y = &uncompressed_pubkey[33..65];
  let prefix = if uncompressed_pubkey_y[31] % 2 == 0 { 2 } else { 3 };
  let mut pubkey = Vec::<u8>::new();
  pubkey.extend_from_slice(&[prefix]);
  pubkey.extend_from_slice(uncompressed_pubkey_x);

  let pubkey_hashed_1 = &Sha256::digest(&pubkey);
  let pubkey_hashed_2 = &Ripemd160::digest(&pubkey_hashed_1);
  let canonical_address = CanonicalAddr::from(&pubkey_hashed_2[..]);
  let humanized_address = deps.api.addr_humanize(&canonical_address).unwrap();
  let bech32_address = bech32::encode("wasm", &pubkey_hashed_2.to_base32(), bech32::Variant::Bech32).unwrap();

  let resp = QueryPublicKeyResp {
    uncompressed_pubkey_hex: hex::encode(&uncompressed_pubkey[..]),
    uncompressed_pubkey_x_hex: hex::encode(uncompressed_pubkey_x),
    uncompressed_pubkey_y_hex: hex::encode(uncompressed_pubkey_y),
    pubkey_hex: hex::encode(pubkey),
    raw_address_hex: hex::encode(pubkey_hashed_2),
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

  let (signature, recovery_param) = signature.split_at(signature.len()-1);
  let uncompressed_pubkey = deps.api
    .secp256k1_recover_pubkey(&message_hash, &signature, recovery_param[0]).unwrap();
  let uncompressed_pubkey_x = &uncompressed_pubkey[1..33];
  let uncompressed_pubkey_y = &uncompressed_pubkey[33..65];
  let prefix = if uncompressed_pubkey_y[31] % 2 == 0 { 2 } else { 3 };
  let mut pubkey = Vec::<u8>::new();
  pubkey.extend_from_slice(&[prefix]);
  pubkey.extend_from_slice(uncompressed_pubkey_x);

  let pubkey_hashed_1 = &Sha256::digest(&pubkey);
  let pubkey_hashed_2 = &Ripemd160::digest(&pubkey_hashed_1);
  let canonical_address = CanonicalAddr::from(&pubkey_hashed_2[..]);
  let humanized_address = deps.api.addr_humanize(&canonical_address).unwrap();
  let bech32_address = bech32::encode("wasm", &pubkey_hashed_2.to_base32(), bech32::Variant::Bech32).unwrap();

  let resp = QueryPublicKeyRecoveryResp {
    uncompressed_pubkey_hex: hex::encode(&uncompressed_pubkey[..]),
    uncompressed_pubkey_x_hex: hex::encode(uncompressed_pubkey_x),
    uncompressed_pubkey_y_hex: hex::encode(uncompressed_pubkey_y),
    pubkey_hex: hex::encode(pubkey),
    raw_address_hex: hex::encode(pubkey_hashed_2),
    canonical_address: canonical_address.to_string(),
    humanized_address: humanized_address.to_string(),
    bech32_address,
  };
  to_binary(&resp)
}
