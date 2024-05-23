use std::process::exit;

use tiny_http::Server;
use types::FolderTokens;

mod parse;
pub mod types;
pub mod tf;

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
    }

    //let folder_path: &str = "./pages/";
    //let documents: FolderTokens = parse::parse_dir(folder_path, true, true).unwrap();
    //println!("FOLDER: {:?}", documents);

}
