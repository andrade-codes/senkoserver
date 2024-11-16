use std::{convert::Infallible, fs, path::Path};
use hyper::{Body, Request, Response};

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.method() != hyper::Method::GET {
        return Ok(Response::builder()
            .status(405) // Method Not Allowed
            .body(Body::from("Method Not Allowed"))
            .unwrap());
    }

    let path = format!("./www{}", req.uri().path());
    let file_path = Path::new(&path);

    if file_path.exists() && file_path.is_file() {
        match fs::read_to_string(file_path) {
            Ok(contents) => Ok(Response::new(Body::from(contents))),
            Err(_) => Ok(Response::builder()
                .status(500) // Internal Server Error
                .body(Body::from("Internal Server Error"))
                .unwrap()),
        }
    } else {
        Ok(Response::builder()
            .status(404) // Not Found
            .body(Body::from("File Not Found"))
            .unwrap())
    }
} 