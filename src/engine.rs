use std::{borrow::Borrow, collections::HashMap, fs::{File, OpenOptions}, io::{Read, Write}, path::Path, string, sync::{Arc, Mutex}};
use bincode::{config, Decode, Encode};

use anyhow::Error;

#[derive(Clone)]
pub struct Engine {
  pub store: Arc<Mutex<Store>>
}

#[derive(Encode, Decode, Clone)]
pub struct Store {
  pub map: HashMap<String, String>
}

impl Store {
  pub fn new() -> Self {
    return Store{map: HashMap::<String, String>::new()};
  }

  pub fn from_state(state_file: &str) -> Result<Self, Error> {
    let mut file = File::open(state_file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let (decoded_store,len) : (Store, usize) = bincode::decode_from_slice(&buf, config::standard())?;
    if len == 0 {
      return Err(Error::msg("Error decoding store, 0 bytes read"));
    }
    return Ok(decoded_store);
  }

  pub fn save_state(&self, state_file: &str) -> Result<(), Error> {
    let encoded = bincode::encode_to_vec(self, config::standard())?;
    let mut file = OpenOptions::new()
      .create(true)
      .write(true)
      .open(state_file)?;

    file.write_all(&encoded)?;

    return Ok(())
  }
}

impl Engine {
  pub fn from_state_or_empty(state_file: &str) -> Self {
    let store = Store::from_state(state_file)
      .unwrap_or(Store::new());
    return Engine {
      store: Arc::new(Mutex::new(store))
    };
  }

  pub fn from_state(state_file: &str) -> Result<Self, Error> {
    let store = Store::from_state(state_file)?;
    return Ok(Engine {
      store: Arc::new(Mutex::new(store))
    });
  }
}

impl Default for Engine {
  fn default() -> Self {
    return Engine{store: Arc::new(Mutex::new(Store::new()))};
  }
}
#[test]
fn test_state() {
  const STATE_FILE: &str = "state.bin";
  let mut origin = Store::new();
  origin.map.insert(String::from("sample_key"), String::from("sample_value"));
  let save = origin.save_state(STATE_FILE);
  assert!(save.is_ok());

  // State loaded correctly from file
  let engine = Engine::from_state_or_empty(STATE_FILE);
  assert!(!engine.store.lock().unwrap().map.is_empty());

  let no_file = Engine::from_state("doesnt_exist");
  assert!(no_file.is_err())
}
