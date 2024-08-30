use std::env;
use std::path::PathBuf;

pub struct Env {}

impl Env {
  pub fn load() {
    let invoke_path = env::args().nth(0).map(PathBuf::from).unwrap().parent().unwrap().to_path_buf();
    match dotenv::from_path(invoke_path.canonicalize().unwrap().join(".env")) {
      Ok(_) => (),
      Err(_) => {
        dotenv::dotenv().ok();
      }
    }
  }

  pub fn var<T: AsRef<str>>(key: T) -> String {
    let key = key.as_ref();
    match std::env::var(key) {
      Ok(val) => val,
      Err(_) => "".to_string(),
    }
  }

  // pub fn u8(key: String) -> u8 {
  //   match std::env::var(key) {
  //     Ok(val) => val.parse::<u8>().unwrap_or(0),
  //     Err(_) => 0,
  //   }
  // }

  pub fn int<T: AsRef<str>>(key: T) -> i32 {
    let key = key.as_ref();
    match std::env::var(key) {
      Ok(val) => val.parse::<i32>().unwrap_or(0),
      Err(_) => 0,
    }
  }

  // pub fn int64<S: AsRef<str>>(key: S) -> i64 {
  //   match std::env::var(key.as_ref()) {
  //     Ok(val) => val.parse::<i64>().unwrap_or(0),
  //     Err(_) => 0,
  //   }
  // }

  pub fn usize<T: AsRef<str>>(key: T) -> usize {
    let key = key.as_ref();
    match std::env::var(key) {
      Ok(val) => val.parse::<usize>().unwrap_or(0),
      Err(_) => 0,
    }
  }

  pub fn vars<T: AsRef<str>>(key: T) -> Vec<String> {
    let key = key.as_ref();
    let mut vars: Vec<String> = Vec::new();
    let mut i: u32 = 1;
    loop {
      let var = Env::var(format!("{}_{}", key, i));
      if "" == var {
        break;
      }
      vars.push(var);
      i += 1;
    }
    vars
  }
}