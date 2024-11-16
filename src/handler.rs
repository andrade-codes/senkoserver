use crate::files::FileInfo;
use hyper::{Body, Request, Response};
use std::collections::HashMap;
use std::convert::Infallible;

pub async fn handle_request(
    req: Request<Body>,
    files_info: HashMap<String, FileInfo>,
) -> Result<Response<Body>, Infallible> {
    if req.method() != hyper::Method::GET {
        return Ok(Response::builder()
            .status(405) // Method Not Allowed
            .body(Body::from("Method Not Allowed"))
            .unwrap());
    }

    // Obtém a chave diretamente do URI, removendo o prefixo /www
    let key = req.uri().path().to_string();

    if let Some(file_info) = files_info.get(&key) {
        Ok(Response::new(Body::from(file_info.content.clone())))
    } else {
        Ok(Response::builder()
            .status(404) // Not Found
            .body(Body::from("File Not Found"))
            .unwrap())
    }
}