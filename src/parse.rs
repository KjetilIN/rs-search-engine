use std::{collections::HashMap, process::exit};
use std::error::Error;
use regex::Regex;
use walkdir::WalkDir;
use crate::types::{PageInformationMap, Website};
use crate::{file_operations::read_file, types::{FolderTokens, TokenizedDocument}};


pub fn parse_file_html(file_path: &str) -> Result<String, Box<dyn Error>>{
    // Only allow html files to be parsed 
    if !file_path.ends_with(".html"){
        println!("[ERROR] Tried to parse the following file as HTML file: {file_path}");
        return Err(Box::from("Parsing error"));
    }

    // Content of the file 
    let content = read_file(file_path).expect("File not read while parsing html");

    Ok(content)
}

pub fn tokenize_document(content: String) -> Result<TokenizedDocument, Box<dyn Error>>{
    // Create new map for the tokens
    let mut map: TokenizedDocument = HashMap::new();

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

pub fn parse_file_md(file_path: &str) -> Result<Option<TokenizedDocument>, Box<dyn Error>>{
    unimplemented!("Parse Markdown File")
}

pub fn parse_dir(folder_path: &str, log_enabled: bool, exit_on_parse_error: bool) -> Result<(FolderTokens, PageInformationMap), ()> {
    let mut folder_tokens: FolderTokens = HashMap::new();
    let mut page_information_map: PageInformationMap = HashMap::new();

    let walker = WalkDir::new(folder_path).into_iter();

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    // Log the current file
                    if log_enabled {
                        println!("[INFO] Parsing file {}", path.display());
                    }

                    let document_content: String = match parse_file_html(path.to_str().unwrap()) {
                        Ok(value) => value,
                        Err(_) => {
                            if log_enabled {
                                println!("[ERROR] Could not parse file {}", path.display());
                            }

                            if exit_on_parse_error {
                                exit(1);
                            }

                            return Err(());
                        },
                    };

                    if document_content.is_empty(){
                        continue;
                    }

                    // Create the website object
                    let website = Website::from_html(&document_content, &path.to_string_lossy().to_string());
                    let tokens = match tokenize_document(document_content){
                        Ok(value) => value,
                        Err(_) => {
                            if log_enabled {
                                println!("[ERROR] Could not tokenize file {}", path.display());
                            }

                            if exit_on_parse_error {
                                exit(1);
                            }

                            return Err(());
                        },
                    };
                   
                    folder_tokens.insert(path.to_string_lossy().to_string(), tokens);
                    page_information_map.insert(path.to_string_lossy().to_string(), website);

                    
                }
            }
            Err(err) => {
                if log_enabled {
                    eprintln!("[ERROR] Failed to read entry: {}", err);
                }
                if exit_on_parse_error {
                    exit(1);
                }
            }
        }
    }

    Ok((folder_tokens, page_information_map))
}