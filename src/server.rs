use tonic::{transport::Server, Request, Response, Status};
use keyval::{GetValueRequest, GetValueReply, InsertKeyValueRequest, InsertKeyValueResponse};
use keyval::value_server::{Value, ValueServer};
mod engine;

pub mod keyval {
    tonic::include_proto!("keyval"); // The string specified here must match the proto package name
}

#[derive(Default)]
pub struct MyValue {
  engine: engine::Engine,
}

#[tonic::async_trait]
impl Value for MyValue {
  async fn get_value(&self, request: Request<GetValueRequest>) -> Result<Response<GetValueReply>, Status> {
    let reply = GetValueReply{value: format!("value")};

    Ok(Response::new(reply))
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
