use josekit::{
  jwe::{self, Dir, A128GCMKW},
  jws::{JwsHeader, RS256},
  jwt::{self, JwtPayload},
};

use crate::common::*;

#[derive(Default)]
pub struct TokenRepository {}

impl TokenRepository
{
  pub fn uid<T>(token: T) -> Result<String, Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let token = token.as_ref();

    let mut header = JwsHeader::new();
    header.set_token_type("JWT");

    let mut payload = JwtPayload::new();
    payload.set_subject("subject");

    let jwt_key = Env::var("JWT_KEY");
    let decrypter = Dir.decrypter_from_bytes(&jwt_key)?;
    let (payload, header) = jwe::deserialize_compact(&token, &decrypter)?;
    // println!("auth token {token:} {payload:?} {header:?}");

    // let jwt_key = Env::var("JWT_KEY");
    // let decrypter = A128GCMKW.decrypter_from_bytes(&jwt_key)?;
    // let (payload, header) = jwt::decode_with_decrypter(&token, &decrypter)?;
    // println!("jwt_key {jwt_key:?} {payload:?} {header:?}");

    let public_key = std::fs::read(std::env::home_dir().unwrap().join(".ssh/jwt_rsa.pub")).unwrap();
    let verifier = RS256.verifier_from_pem(&public_key)?;
    let (payload, header) = jwt::decode_with_verifier(&payload, &verifier)?;
    println!("auth token {token:} {payload:?} {header:?}");

    Ok("hello".into())
  }
}
