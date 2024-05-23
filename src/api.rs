use tiny_http::{Request, Response};
use crate::file_operations::read_file;

fn serve_public_file(file_name:&str, content_type: &str, request: Request) -> (){
    let file = format!("./src/public/{}", file_name);
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

pub fn handle_bad_request(request: Request) -> (){
    let res = Response::from_string("Bad request").with_status_code(400);
    let _ = request.respond(res);
}

pub fn handle_get_request(request: Request)-> (){
    // Serve based on the url
    match request.url(){
        // INDEX File
        "/" => serve_public_file("index.html", "text/html", request),
        "/style.css" => serve_public_file("style.css", "text/css", request),
        "/script.js" => serve_public_file("script.js", "text/javascript", request),
        _ => handle_bad_request(request)
    }
}

pub fn handle_post_request(request: Request) -> (){
    unimplemented!()
}

