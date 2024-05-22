use std::{collections::HashMap, fs::{self, File}, io::{BufReader, Read}, process::exit};
use std::error::Error;
use regex::Regex;

type TokenizedDocument = HashMap<String, usize>; 
type FolderTokens = HashMap<String, TokenizedDocument>; 

const FOLDER_PATH: &str = "./pages/";

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>>{
    // Read the file 
    let file =  match File::open(file_path){
        Ok(file) => file,
        Err(e) => {
            eprintln!("[ERROR] Was not able to find file {}: {}", file_path, e);
            panic!()
        }
    };

    // Read content to buffer reader 
    let mut content = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut content).unwrap_or_else(|err| {
        eprintln!("[ERROR] could not read file {}: {}", file_path, err);
        panic!()
    });

    // Ok
    Ok(content)
}


fn parse_file_html(file_path: &str) -> Result<TokenizedDocument, Box<dyn Error>>{
    // Create new map for the tokens
    let mut map: TokenizedDocument = HashMap::new();

    // Only allow html files to be parsed 
    if !file_path.ends_with(".html"){
        println!("[ERROR] Tried to parse the following file as HTML file: {file_path}");
        return Err(Box::from("Parsing error"));
    }

    // Content of the file 
    let content = read_file(file_path).expect("File not read while parsing html");

    // Iterate over each file 
    for line in content.lines(){
        let tag_regex: Regex = Regex::new(r"<[^>]*>").unwrap();
        let line_to_tokenize: String = tag_regex.replace_all(&line, "").into_owned();
        
        let tokens: Vec<&str> = line_to_tokenize.split_whitespace().collect();

   
        for token in tokens{
            let counter = map.entry(token.to_ascii_lowercase()).or_insert(0);
            *counter += 1;
        }
    }

    Ok(map)
}


fn parse_file_md(file_path: &str) -> Result<Option<TokenizedDocument>, Box<dyn Error>>{
    unimplemented!("Parse Markdown File")
}

fn parse_dir(folder_path: &str, log_enabled: bool, exit_on_parse_error: bool) -> Result<FolderTokens, ()> {
    let mut folder_tokens = HashMap::new();

    let paths = fs::read_dir(folder_path).unwrap_or_else(|err| {
        eprintln!("[ERROR] Was not able to read dir path {folder_path}:{err}");
        exit(1)
    });

    for path in paths{
        if let Some(current_path) = path.unwrap().path().to_str(){
            // Log the current file 
            if log_enabled{
                println!("[INFO] Parsing file {}", current_path);
            }
                
            let document_tokens: TokenizedDocument = match parse_file_html(current_path){
                Ok(value) => value,
                Err(_) =>{
                    if log_enabled{
                        println!("[ERROR] Could not parse file {}", current_path);
                    }

                    if exit_on_parse_error{
                        exit(1)
                    }

                    return Ok(HashMap::new());
                },
            };

            if document_tokens.is_empty(){
                continue;
            }else{
                folder_tokens.insert(current_path.to_string(), document_tokens);
            }

        }else{
            if log_enabled{
                eprintln!("[ERROR] File not available");
            }
            if exit_on_parse_error{
                exit(1)
            }
        }
    }
    
    Ok(folder_tokens)
}

fn main() {
    
    let documents: FolderTokens = parse_dir(FOLDER_PATH, true, true).unwrap();

    println!("FOLDER: {:?}", documents);

}
