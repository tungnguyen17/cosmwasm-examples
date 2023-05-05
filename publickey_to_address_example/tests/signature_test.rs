use cosmwasm_std::{
  Addr,
};
use cw_multi_test::{
  App,
  ContractWrapper,
  Executor,
};
use publickey_to_address::{
  execute,
  instantiate,
  msg::{
    InstantiateMsg,
    QueryMsg,
    QueryPublicKeyRecoveryResp,
  },
  query,
};
use secp256k1::{
  ecdsa::{
    RecoveryId,
    RecoverableSignature,
  },
  KeyPair,
  Message,
  Secp256k1,
  SecretKey,
};
use sha2::{
  Digest,
  Sha256,
};

#[test]
fn signature_recovery_ecdsa_test() {
  let testcase = "signature_recovery_ecdsa_test";

  let mut app = App::default();
  let code = ContractWrapper::new(execute, instantiate, query);
  let code_id = app.store_code(Box::new(code));

  let addr = app.instantiate_contract(
    code_id,
    Addr::unchecked("wasm102nulyepw0hldz5ja5afyuzjch7u9cgk4fq5t9"),
    &InstantiateMsg {},
    &[],
    "wasm17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgsm0v070",
    None,
  ).unwrap();

  let message: Vec<u8> = vec!(1,2,3,4);
  let signer = default_keypair();
  let (message_hash, signature) = sign_message(message, signer);
  let resp: QueryPublicKeyRecoveryResp = app.wrap()
    .query_wasm_smart(addr, &QueryMsg::QueryPublicKeyRecovery { message_hash, signature: signature.clone() })
    .unwrap();
  let (signature, recovery_param) = signature.split_at(signature.len()-1);
  println!("{}: keypair_pubkey = {}", testcase, signer.public_key());
  println!("{}: signature = {} ", testcase, hex::encode(&signature));
  println!("{}: recovery_param = {}", testcase, recovery_param[0]);
  println!("{}: response_uncompressed_pubkey = {}", testcase, resp.uncompressed_pubkey_hex);
  println!("{}: response_uncompressed_pubkey_x = {}", testcase, resp.uncompressed_pubkey_x_hex);
  println!("{}: response_uncompressed_pubkey_y = {}", testcase, resp.uncompressed_pubkey_y_hex);
  println!("{}: response_pubkey = {}", testcase, resp.pubkey_hex);
  println!("{}: response_raw_address = {}", testcase, resp.raw_address_hex);
  println!("{}: response_canonical_address = {}", testcase, resp.canonical_address);
  println!("{}: response_humanized_address = {}", testcase, resp.humanized_address);
  println!("{}: response_bech32_address = {}", testcase, resp.bech32_address);
}

fn default_keypair(
) -> KeyPair {
  let seed =  [196, 195, 174, 168, 97, 1, 251, 160, 72, 5, 138, 97, 67, 30, 4, 118, 25, 109, 95, 53, 46, 135, 217, 52, 206, 176, 90, 162, 131, 204, 168, 103];
  let secret_key = SecretKey::from_slice(&seed).unwrap();
  KeyPair::from_secret_key(&Secp256k1::new(), &secret_key)
}

fn sign_message(
  message: Vec<u8>,
  signer: KeyPair,
) -> (Vec<u8>, Vec<u8>) {
  let message_hash: &[u8] = &Sha256::digest(&message);
  let message_to_sign = Message::from_slice(message_hash).unwrap();
  let ecdsa = Secp256k1::signing_only();
  let signature: RecoverableSignature = ecdsa.sign_ecdsa_recoverable(&message_to_sign, &signer.secret_key());
  let (recovery_param, signature_bytes): (RecoveryId, [u8; 64]) = signature.serialize_compact();
  let recovery_param = if recovery_param.to_i32() == 0 { 0u8 } else { 1u8 };
  let mut signature_bytes = signature_bytes.to_vec();
  signature_bytes.extend_from_slice(&[recovery_param]);
  (message_hash.to_vec(), signature_bytes)
}
