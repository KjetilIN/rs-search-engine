use std::{collections::HashMap, fmt::Error, fs::{self, File}, io::{BufReader, Read}, process::exit};

use regex::Regex;

type TokenizedDocument = HashMap<String, usize>; 
type FolderTokens = HashMap<String, TokenizedDocument>; 

const FOLDER_PATH: &str = "./pages/";


fn parse_file_html(file_path: &str) -> Result<TokenizedDocument, Error>{
    // Create new map for the tokens
    let mut map: TokenizedDocument = HashMap::new();

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

    // Iterate over each file 
    for line in content.lines(){
        let tag_regex: Regex = Regex::new(r"<[^>]*>").unwrap();
        let line_to_tokenize: String = tag_regex.replace_all(&line, "").into_owned();
        
        let tokens: Vec<&str> = line_to_tokenize.split(" ")
                                                .filter(|s| !s.trim().is_empty())
                                                .collect();  

        if tokens.len() > 0{
            println!("{:?}", tokens); 
        }
        
    }

    Ok(map)
}


fn parse_file_md(file_path: &str) -> Result<Option<TokenizedDocument>, Error>{
    unimplemented!("Parse Markdown File")
}

fn parse_dir(folder_path: &str) -> Result<FolderTokens, ()> {
    let mut folder_tokens = HashMap::new();

    let paths = fs::read_dir(folder_path).unwrap_or_else(|err| {
        eprintln!("[ERROR] Was not able to read dir path {folder_path}:{err}");
        exit(1)
    });

    for path in paths{
        if let Some(current_path) = path.unwrap().path().to_str(){
            println!("[INFO] Parsing file {}", current_path);
            _ = parse_file_html(current_path);
        }else{
            eprintln!("[ERROR] File not available");
            exit(1)
        }
    }
    
    Ok(folder_tokens)
}

fn main() {
    
    _ = parse_dir(FOLDER_PATH);

}
