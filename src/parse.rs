use std::sync::{Arc, Mutex};
use std::{collections::HashMap, process::exit};
use std::error::Error;
use regex::Regex;
use walkdir::WalkDir;
use crate::types::{PageInformationMap, Website};
use rayon::prelude::*;
use crate::{file_operations::read_file, types::{FolderTokens, TokenizedDocument}};


const URLS_PATH: &str = "./cache/urls.txt";

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

/// Multithreaded dir parser using rayon
/// 
/// - folder_path => path of the directory to be parsed
/// - log_enabled => set to True for logging
/// - exit_on_parse_error => set to true if exit on any error
/// 
/// Return a Result of tuple of the `FoldersTokens` and `PageInformation` if the parsing went okay
pub fn parse_dir(folder_path: &str, log_enabled: bool, exit_on_parse_error: bool) -> Result<(FolderTokens, PageInformationMap), ()> {
    // Create hashmaps that are going to be shared resources
    // - Arc => for shared ownership without copying the code
    // - Mutex => for making sure only one thread can access at the time
    let folder_tokens = Arc::new(Mutex::new(HashMap::new()));
    let page_information_map = Arc::new(Mutex::new(HashMap::new()));
    let urls = read_urls_file(&URLS_PATH).unwrap();

    let walker = WalkDir::new(folder_path).into_iter();

    // Use rayon's parallel iterator to multithread the parsing og the files 
    walker
        .filter_map(Result::ok)
        .par_bridge()
        .for_each(|entry| {
            let path = entry.path();
            if path.is_file() {
                // Log the current file
                if log_enabled {
                    println!("[INFO] Parsing file {}", path.display());
                }

                // Get the file number
                let url_index = match path
                    .to_string_lossy()
                    .strip_prefix("./pages/file")
                    .and_then(|s| s.strip_suffix(".html"))
                    .and_then(|s| s.parse::<usize>().ok()) 
                {
                    Some(index) => index,
                    None => {
                        if log_enabled {
                            println!("[ERROR] Invalid file name format {}", path.display());
                        }
                        if exit_on_parse_error {
                            exit(1);
                        }
                        return;
                    }
                };

                let document_content: String = match parse_file_html(path.to_str().unwrap()) {
                    Ok(value) => value,
                    Err(_) => {
                        if log_enabled {
                            println!("[ERROR] Could not parse file {}", path.display());
                        }
                        if exit_on_parse_error {
                            exit(1);
                        }
                        return;
                    },
                };

                if document_content.is_empty() {
                    return;
                }

                // Create the website object
                let url: &String = &urls[url_index - 1];
                let website = Website::from_html(&document_content, &url);
                let tokens = match tokenize_document(document_content) {
                    Ok(value) => value,
                    Err(_) => {
                        if log_enabled {
                            println!("[ERROR] Could not tokenize file {}", path.display());
                        }
                        if exit_on_parse_error {
                            exit(1);
                        }
                        return;
                    },
                };

                // Insert into shared collections
                {
                    let mut folder_tokens = folder_tokens.lock().unwrap();
                    folder_tokens.insert(path.to_string_lossy().to_string(), tokens);
                }
                {
                    let mut page_information_map = page_information_map.lock().unwrap();
                    page_information_map.insert(path.to_string_lossy().to_string(), website);
                }
            }
        });

        let folder_tokens = Arc::try_unwrap(folder_tokens).expect("Arc still has multiple owners").into_inner().expect("Mutex cannot be locked");
        let page_information_map = Arc::try_unwrap(page_information_map).expect("Arc still has multiple owners").into_inner().expect("Mutex cannot be locked");
    

    Ok((folder_tokens, page_information_map))
}

pub fn read_urls_file(file_path:&str) -> Result<Vec<String>, ()>{
    let mut vec: Vec<String> = Vec::new();
    let file = read_file(file_path);
    if let Ok(content) = file{
        for line in content.lines(){
            let tokens: Vec<&str> = line.split(";").into_iter().collect();
            if tokens.len() != 2{
                return Err(())
            }
            vec.push(tokens[0].to_string())
        }
        
        return Ok(vec);
    }
    Err(())
}