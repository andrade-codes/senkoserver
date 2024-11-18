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

    // Inicializa o HashMap com os arquivos coletados
    let files_info = Arc::new(Mutex::new(collect_files_info(path).unwrap_or_default()));

    // Clona `files_info` para o watcher
    let files_info_clone = files_info.clone();

    // Inicia o monitoramento dos arquivos e mantém o watcher vivo
    let watcher = watch_files(
        path,
        move |updated_files_info: &HashMap<String, FileInfo>| {
            let mut files_info = files_info_clone.lock().unwrap();
            *files_info = updated_files_info.clone();

            // Log para verificar se o callback está sendo chamado
            println!("Files updated in main: {:?}", files_info.keys());
        },
    )
    .expect("Failed to start file watcher");

    // Mantém o watcher vivo armazenando-o em uma variável
    // Isso impede que o watcher seja descartado e pare de monitorar os arquivos
    std::mem::forget(watcher);

    // Configura o servidor HTTP
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

    // Inicia o servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 8787));
    let server = Server::bind(&addr).serve(make_svc);

    println!("server is running on {:?}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
