use josekit::jwe::{self, enc::A256GCM, RSA_OAEP_256};

#[derive(Default)]
pub struct JweRepository {}

impl JweRepository
{
  pub fn encrypt(payload: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut header =  jwe::JweHeader::new();
    header.set_content_encryption(A256GCM.name());
    let public_key = std::fs::read(home::home_dir().unwrap().join(".ssh/client_rsa.pub")).unwrap();
    let encrypter = RSA_OAEP_256.encrypter_from_pem(&public_key)?;
    let payload = jwe::serialize_compact(payload, &header, &encrypter)?;
    Ok(payload)
  }

  pub fn decrypt<T>(payload: T) -> Result<Vec<u8>, Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let payload = payload.as_ref();
    let private_key = std::fs::read(home::home_dir().unwrap().join(".ssh/jwe_rsa")).unwrap();
    let decrypter = RSA_OAEP_256.decrypter_from_pem(&private_key)?;
    let (payload, _) = jwe::deserialize_compact(payload, &decrypter)?;
    Ok(payload)
  }
}
