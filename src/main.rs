use std::{convert::Infallible, net::SocketAddr};
use hyper::{service::{make_service_fn, service_fn}, Body, Request, Response, Server};
use handler::handle_request; // Importando a função do módulo handler

mod handler; // Declarando o módulo handler

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        async {
            Ok::<_, Infallible>(service_fn(handle_request))
        }
    });
    let addr = SocketAddr::from(([127, 0, 0, 1], 8787));
    let server = Server::bind(&addr).serve(make_svc);
    println!("server is running on {:?}", addr);
    if let Err(e) = server.await {
        println!("error : {}", e);
    }
}