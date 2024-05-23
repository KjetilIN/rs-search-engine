use std::process::exit;
use tiny_http::{Method, Server};

use crate::api::{handle_bad_request, handle_get_request, handle_post_request};

mod parse;
pub mod types;
pub mod tf;
pub mod api;
pub mod file_operations;

fn main() {

    let addr = "127.0.0.1:8080";
    let server = Server::http(addr).unwrap_or_else(|err| {
        eprintln!("[ERROR] Could not start HTTP server on {addr}: {err}");
        exit(1)
    });

    println!("[INFO] Serving a HTTP server on {addr}");

    loop {
        // Read request from server
        let request = match server.recv(){
            Ok(req) => req,
            Err(err) => {
                eprintln!("[ERROR] Could not handle request: {err}");
                continue;
            } 
        }; 
        println!("[INFO] URL {}", request.url()); 

        // Handle requests based on methods
        match request.method() {
            Method::Get => handle_get_request(request),
            Method::Post => handle_post_request(request),
            _ => handle_bad_request(request),
        }   
    }

    //let folder_path: &str = "./pages/";
    //let documents: FolderTokens = parse::parse_dir(folder_path, true, true).unwrap();
    //println!("FOLDER: {:?}", documents);

}
