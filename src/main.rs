use files::collect_files_info;
use handler::handle_request;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use std::{convert::Infallible, net::SocketAddr};

mod files;
mod handler;

#[tokio::main]
async fn main() {
    // Coletar informações dos arquivos

    let path = "www";
    let files_info = collect_files_info(path).unwrap_or_default();

    //print files_info
    // for (key, value) in files_info.iter() {
    //     println!("{}: {:?}", key, value.hash);
    // }

    let make_svc = make_service_fn(move |_conn| {
        let files_info = files_info.clone(); // Clonando para uso no handler
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, files_info.clone())
            }))
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 8787));
    let server = Server::bind(&addr).serve(make_svc);
    println!("server is running on {:?}", addr);
    if let Err(e) = server.await {
        println!("error : {}", e);
    }
}
