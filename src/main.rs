use files::{collect_files_info, watch_files, FileInfo};
use handler::handle_request;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{
    collections::HashMap,
    convert::Infallible,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

mod files;
mod handler;

#[tokio::main]
async fn main() {
    let path = "www";

    let files_info = Arc::new(Mutex::new(collect_files_info(path).unwrap_or_default()));

    let files_info_clone = files_info.clone();

    let watcher = watch_files(
        path,
        move |updated_files_info: &HashMap<String, FileInfo>| {
            let mut files_info = files_info_clone.lock().unwrap();
            *files_info = updated_files_info.clone();
        },
    )
    .expect("Failed to start file watcher");

    std::mem::forget(watcher);

    let make_svc = make_service_fn(move |_conn| {
        let files_info = files_info.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let files_info = files_info.clone();

                async move {
                    let files_info = files_info.lock().unwrap().clone();
                    let result = handle_request(req, files_info).await;

                    match result {
                        Ok(response) => Ok::<_, Infallible>(response),
                        Err(_) => Ok::<_, Infallible>(
                            Response::builder()
                                .status(500)
                                .body(Body::from("Internal Server Error"))
                                .unwrap(),
                        ),
                    }
                }
            }))
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 8787));
    let server = Server::bind(&addr).serve(make_svc);

    println!("server is running on {:?}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
