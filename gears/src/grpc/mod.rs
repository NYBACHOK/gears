use std::{convert::Infallible, net::SocketAddr};

use tonic::{
    body::BoxBody,
    server::NamedService,
    transport::{server::Router, Body},
};
use tower_layer::Identity;
use tower_service::Service;

use crate::runtime::runtime;

mod error;
pub mod health;
pub mod tx;

pub fn run_grpc_server(router: Router<Identity>, address: Option<SocketAddr>) {
    std::thread::spawn(move || {
        let result = runtime().block_on(launch(router, address));
        if let Err(err) = result {
            panic!("Failed to run gRPC server with err: {}", err)
        }
    });
}

#[allow(dead_code)]
trait GService:
    Service<
        http::Request<Body>,
        Response = http::Response<BoxBody>,
        Error = Infallible,
        Future = dyn Send + 'static,
    > + NamedService
    + Clone
    + Send
    + 'static
{
}

async fn launch(
    router: Router<Identity>,
    address: Option<SocketAddr>,
) -> Result<(), Box<dyn std::error::Error>> {
    let address = address.unwrap_or(
        "127.0.0.1:8080"
            .parse()
            .expect("hard coded address is valid"),
    );

    tracing::info!("gRPC server running at {}", address);
    router.serve(address).await?;
    Ok(())
}
