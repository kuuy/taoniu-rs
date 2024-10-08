use std::time::SystemTime;

use josekit::{
  jwe::{self, Dir},
  jws::{JwsHeader, RS256},
  jwt::{self, JwtPayload}
};
use serde_json::Value;

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
    let (payload, _) = jwe::deserialize_compact(&token, &decrypter)?;

    let public_key = std::fs::read(home::home_dir().unwrap().join(".ssh/jwt_rsa.pub")).unwrap();
    let verifier = RS256.verifier_from_pem(&public_key)?;
    let (payload, _) = jwt::decode_with_verifier(&payload, &verifier)?;

    let now = SystemTime::now();
    let _ = match payload.expires_at() {
      Some(expires_at) => {
        if expires_at <= now {
          return Err(Box::from("token has been expired"))
        }
      }
      _ => return Err(Box::from("invalid token"))
    };

    let uid = match payload.claim("uid") {
      Some(uid) => {
        match uid {
          Value::String(uid) => uid.as_str(),
          _ => return Err(Box::from("invalid token"))
        }
      }
      None => return Err(Box::from("invalid token"))
    };

    Ok(uid.into())
  }
}
