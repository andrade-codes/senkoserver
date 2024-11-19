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
    sync::{Arc, RwLock},
};

mod files;
mod handler;

#[tokio::main]
async fn main() {
    let path = "www";

    // Inicializa o estado dos arquivos
    let files_info = Arc::new(RwLock::new(collect_files_info(path).unwrap_or_default()));

    // Clona o Arc para o watcher
    let files_info_clone = Arc::clone(&files_info);

    // Inicia o watcher para monitorar mudanças nos arquivos
    let watcher = watch_files(
        path,
        move |updated_files_info: &HashMap<String, FileInfo>| {
            let mut files_info = files_info_clone.write().unwrap(); // Obtém o bloqueio de escrita
            *files_info = updated_files_info.clone(); // Atualiza o estado compartilhado
                                                      // println!("Updated files_info in RwLock: {:?}", files_info.keys()); // Log de depuração
        },
    )
    .expect("Failed to start file watcher");

    // Garante que o watcher não será finalizado prematuramente
    std::mem::forget(watcher);

    // Define o serviço HTTP
    let make_svc = make_service_fn(move |_conn| {
        let files_info = Arc::clone(&files_info); // Clona o Arc para o closure

        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let files_info = Arc::clone(&files_info); // Clona novamente para o closure interno

                async move {
                    // Chama o handler diretamente com o estado mais recente
                    let result = handle_request(req, &files_info).await;

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

    // Configura o endereço do servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 8787));
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server is running on {:?}", addr);

    // Inicia o servidor
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
