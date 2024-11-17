use files::{collect_files_info, watch_files};
use handler::handle_request;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
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

    // Inicializa o HashMap com os arquivos coletados
    let files_info = Arc::new(Mutex::new(collect_files_info(path).unwrap_or_default()));

    // Clona `files_info` para passar para a task de monitoramento
    let files_info_clone = files_info.clone();

    // Lança a task para monitorar mudanças nos arquivos
    tokio::spawn(async move {
        if let Err(err) = watch_files(path, move |updated_files_info| {
            let mut files_info = files_info_clone.lock().unwrap();
            *files_info = updated_files_info;
            println!("Files updated!");
        })
        .await
        {
            eprintln!("Error watching files: {}", err);
        }
    });

    // Configura o servidor HTTP
    let make_svc = make_service_fn(move |_conn| {
        let files_info = files_info.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let files_info = files_info.lock().unwrap().clone();
                handle_request(req, files_info)
            }))
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 8787));
    let server = Server::bind(&addr).serve(make_svc);

    println!("server is running on {:?}", addr);

    // Inicia o servidor
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
