use tiny_http::{Request, Response};
use crate::file_operations::read_file;
use rand::random;

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

pub fn handle_post_request(mut request: Request) -> (){
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
            println!("[INFO] POST Request: {content}");

            // Mocked search
            let urls: Vec<String> = (0..3)
                .map(|_| format!("http://{}.com/{}", content, random::<u32>()))
                .collect();

            println!("URLS: {:?}", urls);

            // Manually creating json format of the results
            let json = format!(
                "{{\"urls\": [\"{}\"]}}",
                urls.join("\",\"")
            );

            println!("JSON: {json}");

            // Creating response object
            let response = Response::from_string(json)
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

