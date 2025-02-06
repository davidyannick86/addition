use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder as ReflectionBuilder;

pub mod calculator {
    tonic::include_proto!("calculator");
}

use calculator::calculator_server::{Calculator, CalculatorServer};
use calculator::SumRequest;
use calculator::SumResponse;

fn addition(a: i32, b: i32) -> String {
    let sum = a + b;
    format!("{} + {} = {}", a, b, sum)
}

#[derive(Default, Debug)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        req: Request<SumRequest>,
    ) -> Result<Response<SumResponse>, Status> {
        let req = req.into_inner();

        let reply = SumResponse {
            result: addition(req.a, req.b),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calculator_service = CalculatorService::default();

    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(include_bytes!(
            "../proto_descriptor.bin"
        ))
        .build_v1()?;

    println!("Microservice Calculator en écoute sur {}", addr);

    Server::builder()
        .add_service(CalculatorServer::new(calculator_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
