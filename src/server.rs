use std::panic::resume_unwind;

use bincode::de;
use tonic::{transport::Server, Request, Response, Status};
use keyval::{GetValueRequest, GetValueReply, InsertKeyValueRequest, InsertKeyValueResponse};
use keyval::value_server::{Value, ValueServer};
mod engine;

pub mod keyval {
  tonic::include_proto!("keyval"); 
}

#[derive(Default)]
pub struct MyValue {
  engine: engine::Engine,
}

#[tonic::async_trait]
impl Value for MyValue {
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
        return Result::Err(Status::not_found("Key not found"));
      }
    }
  }

  async fn insert_key_value(&self, request: Request<InsertKeyValueRequest>) -> Result<Response<InsertKeyValueResponse>, Status> {
    let reply = InsertKeyValueResponse{result: format!("value")};
    Ok(Response::new(reply))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;
  let greeter = MyValue::default();

  Server::builder()
    .add_service(ValueServer::new(greeter))
    .serve(addr)
    .await?;

  Ok(())
}
