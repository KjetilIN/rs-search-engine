use std::process::exit;

use serde_json::json;
use tiny_http::{Method, Request, Response, Server};
use crate::{file_operations::{load_from_file, read_file}, tf::search_term, types::{FolderTokens, PageInformationMap, Website}};

fn serve_public_file(file_name:&str, content_type: &str, request: Request) -> (){
    let file = format!("./frontend/{}", file_name);
    let content = read_file(&file);

    match content{
        Ok(ct) => {
            let response = Response::from_string(ct)
                                                                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &content_type.as_bytes()[..]).unwrap());
                    
            let _ = request.respond(response);
            return; 
        },
        Err(_) => {
            eprintln!("[ERROR] Could not read {}", file_name);
        },
    }
}

pub fn handle_bad_request(request: Request) {
    let response = Response::from_string("Bad Request").with_status_code(400);
    if let Err(err) = request.respond(response) {
        eprintln!("[ERROR] Failed to send bad request response: {}", err);
    }
}

pub fn handle_ok_request(request: Request){
    let response = Response::from_string("Success").with_status_code(200);
    if let Err(err) = request.respond(response) {
        eprintln!("[ERROR] Failed to send bad request response: {}", err);
    }
}

pub fn handle_get_request(request: Request)-> (){
    // Serve based on the url
    match request.url(){
        // INDEX File
        "/" => serve_public_file("index.html", "text/html", request),
        "/result" => serve_public_file("result.html", "text/html", request),
        "/style.css" => serve_public_file("style.css", "text/css", request),
        "/script.js" => serve_public_file("script.js", "text/javascript", request),
        "/result.js" => serve_public_file("result.js", "text/javascript", request),
        _ => handle_bad_request(request)
    }
}

pub fn handle_post_request(mut request: Request, tokens: &FolderTokens, page_information: &PageInformationMap) -> (){
    match request.url() {
        "/api/search" => {
            let mut content = String::new();
            match request.as_reader().read_to_string(&mut content){
                Ok(_) => (),
                Err(err) => {
                    eprintln!("[ERROR] Not able to parse POST body: {err}");
                    handle_bad_request(request);
                    return;
                },
            }
            println!("[INFO] POST Request Searched: '{content}'");

            let search_results: Vec<Website> = match search_term(&content, tokens, page_information){
                Ok(val) => val,
                Err(_) => {
                    eprintln!("[ERROR] Server was not able to search for term");
                    return;
                },
            };

            // Serialize to JSON
            let json = json!({
                "results": search_results
            });
            // Convert the JSON value to a string
            let json_string = serde_json::to_string(&json).expect("Failed to serialize to JSON");

            // Creating response object
            let response = Response::from_string(json_string)
                                                                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &"application/json".as_bytes()[..]).unwrap())
                                                                .with_status_code(200);

            // Respond
            if let Err(err) = request.respond(response) {
                eprintln!("[ERROR] Failed to send bad request response: {}", err);
            }
            return;
            
        },
        _ => handle_bad_request(request)
    }
}


pub fn serve_website(){
    let addr = "127.0.0.1:8080";
    let server = Server::http(addr).unwrap_or_else(|err| {
        eprintln!("[ERROR] Could not start HTTP server on {addr}: {err}");
        exit(1)
    });

    println!("[INFO] Serving a HTTP server on {addr}");

    println!("[INFO] Loading tokens...");

    // Load the files as Hashmaps
    let folder_tokens: FolderTokens = match load_from_file("tokens.dat".to_string()){
        Ok(val) => val,
        Err(err) => {
            eprintln!("[ERROR] Tokenized document not parsed (token.dat): {err}");
            exit(1);
        },
    };
    println!("[INFO] Tokens loaded");

    println!("[INFO] Loading lookup table...");
    let page_information: PageInformationMap = match load_from_file("page_lookup.dat".to_string()) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("[ERROR] Page Information not parsed (page_lookup.dat): {err}");
            exit(1);
        },
        
    };
    println!("[INFO] Page information loaded");

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
            Method::Post => handle_post_request(request, &folder_tokens, &page_information),
            _ => handle_bad_request(request),
        }   
    }
}