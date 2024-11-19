use crate::files::FileInfo;
use hyper::{Body, Request, Response};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, RwLock};

pub async fn handle_request(
    req: Request<Body>,
    files_info: &Arc<RwLock<HashMap<String, FileInfo>>>,
) -> Result<Response<Body>, Infallible> {
    if req.method() != hyper::Method::GET {
        return Ok(Response::builder()
            .status(405)
            .body(Body::from("Method Not Allowed"))
            .unwrap());
    }

    let mut key = req.uri().path().to_string();
    if key.ends_with('/') {
        key.push_str("index.html");
    }

    // Obtém o estado mais recente no momento exato
    let file_info = {
        let files_info = files_info.read().unwrap(); // Obtém o bloqueio de leitura
                                                     // println!("Files available in handler: {:?}", files_info.keys()); // Log de depuração
        files_info.get(&key).cloned() // Obtém o conteúdo atualizado
    };

    if let Some(file_info) = file_info {
        // Servindo o conteúdo atualizado
        Ok(Response::new(Body::from(file_info.content.clone())))
    } else {
        Ok(Response::builder()
            .status(404)
            .body(Body::from("File Not Found"))
            .unwrap())
    }
}
