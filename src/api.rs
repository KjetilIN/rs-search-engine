use tiny_http::{Request, Response};
use crate::file_operations::read_file;

fn serve_public_file(file_name:&str, meta: &str, request: Request) -> (){
    let file = format!("./src/public/{}", file_name);
    let content = read_file(&file);

    match content{
        Ok(ct) => {
            let response = Response::from_string(ct)
                                                                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &meta.as_bytes()[..]).unwrap());
                    
            let _ = request.respond(response);
            return; 
        },
        Err(_) => {
            eprintln!("[ERROR] Could not read {}", file_name);
        },
    }
}

pub fn handle_get_request(request: Request)-> (){
    // Serve based on the url
    match request.url(){
        // INDEX File
        "/" => serve_public_file("index.html", "text/html", request),
        // Serving the styles request 
        "/style.css" => serve_public_file("style.css", "text/css", request),
        _ => {}
    }
}

pub fn handle_post_request(request: Request) -> (){
    unimplemented!()
}

pub fn handle_bad_request(request: Request) -> (){
    unimplemented!()
}