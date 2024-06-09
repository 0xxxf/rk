use engine::Engine;
use tokio::{task, time};
use tonic::{transport::Server, Request, Response, Status};
use keyval::{GetValueRequest, GetValueReply, InsertKeyValueRequest, InsertKeyValueResponse};
use keyval::value_server::{Value, ValueServer};
mod engine;

pub mod keyval {
  tonic::include_proto!("keyval"); 
}

#[derive(Default, Clone)]
pub struct KeyValueService {
  engine: engine::Engine,
}

impl KeyValueService {
  fn with_preload() -> Self {
    return KeyValueService{
      engine: Engine::from_state_or_empty("./state.bin")
    };
  }
}

#[tonic::async_trait]
impl Value for KeyValueService {
  async fn get_value(&self, request: Request<GetValueRequest>) -> Result<Response<GetValueReply>, Status> {
    let request = request.into_inner();
    let key = request.key;
    let map = &self.engine.store.lock().unwrap().map;
    let value = map.get(&key);

    match value {
      Some(r) => {
        let reply = GetValueReply { value: r.to_string() };
        return Ok(Response::new(reply));
      }, 
      None => {
        return Result::Err(Status::not_found("Key not found"))
      }
    }
  }

  async fn insert_key_value(&self, request: Request<InsertKeyValueRequest>) -> Result<Response<InsertKeyValueResponse>, Status> {
    let reply = InsertKeyValueResponse{result: format!("value")};
    let request = request.into_inner();
    let key = request.key;
    let value = request.value;
    self.engine.store.lock().unwrap().map.insert(key, value);
    Ok(Response::new(reply))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;
  let greeter = KeyValueService::with_preload();
  let greeter_clone = greeter.clone();

  let task = task::spawn(async move {
    loop {
      let mut interval = time::interval(time::Duration::from_secs(5 * 60)); 
      interval.tick().await;
      let _ = greeter_clone.engine.store.lock().unwrap().save_state("./state.bin");
    }
  });

  Server::builder()
    .add_service(ValueServer::new(greeter))
    .serve(addr)
    .await?;

  Ok(())
}
