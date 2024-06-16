use crate::page_information::Website;
use crate::types::PageInformationMap;
use crate::{
    file_operations::read_file,
    types::{FolderTokens, TokenizedDocument},
};
use rayon::prelude::*;
use regex::Regex;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, process::exit};
use walkdir::WalkDir;

const URLS_PATH: &str = "./cache/urls.txt";
const MAX_PARAGRAPH_LINES_COUNTER: usize = 3;

/// Parses the given `.html` file based on the given path.
/// 
/// Uses regex to remove all HTML tags.
/// Returns the parsed content or the error that occurred when reading the file
pub fn parse_file_html(file_path: &str) -> Result<String, Box<dyn Error>> {
    // Only allow html files to be parsed
    if !file_path.ends_with(".html") {
        println!("[ERROR] Tried to parse the following file as HTML file: {file_path}");
        return Err(Box::from("Parsing error"));
    }

    // Content of the file
    let content = read_file(file_path).expect("File not read while parsing html");

    let mut parsed_content: String = String::new();

    // Parse the content
    for line in content.lines() {
        let tag_regex: Regex = Regex::new(r"<[^>]*>").unwrap();
        let new_line: String = tag_regex.replace_all(&line, "").into_owned();
        parsed_content += &(new_line + "\n");
    }

    Ok(parsed_content)
}

/// Count the frequency of each term in given string 
/// 
/// Returns a `TokenizedDocument` 
pub fn tokenize_document(content: String) -> TokenizedDocument{
    // Create new map for the tokens
    let mut map: TokenizedDocument = HashMap::new();

    // Iterate over each file
    for line in content.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        for token in tokens {
            let counter = map.entry(token.to_ascii_lowercase()).or_insert(0);
            *counter += 1;
        }
    }

    map
}

pub fn parse_file_md(_file_path: &str) -> Result<Option<TokenizedDocument>, Box<dyn Error>> {
    unimplemented!("Parse Markdown File")
}

/// Multithreaded directory parser using rayon. 
/// 
/// Used to index the whole directory.
/// - `folder_path` path of the directory to be parsed
/// - `log_enabled` set to True for logging
/// - `exit_on_parse_error` set to true if exit on any error
///
/// Return a Result of tuple of the `FoldersTokens` and `PageInformation` if the parsing went okay
pub fn parse_dir(
    folder_path: &str,
    log_enabled: bool,
    exit_on_parse_error: bool,
) -> Result<(FolderTokens, PageInformationMap), ()> {
    // Create hashmaps that are going to be shared resources
    // - Arc => for shared ownership without copying the code
    // - Mutex => for making sure only one thread can access at the time
    let folder_tokens: Arc<Mutex<FolderTokens>> = Arc::new(Mutex::new(HashMap::new()));
    let page_information_map: Arc<Mutex<PageInformationMap>> = Arc::new(Mutex::new(HashMap::new()));

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
                    }
                };

                if document_content.is_empty() {
                    return;
                }

                // Create the website object
                let url: &String = &urls[url_index - 1];
                let website = Website::from_html(&document_content, &url, &path.display().to_string());
                let tokens: TokenizedDocument = tokenize_document(document_content);

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

    let folder_tokens = Arc::try_unwrap(folder_tokens)
        .expect("Arc still has multiple owners")
        .into_inner()
        .expect("Mutex cannot be locked");
    let page_information_map = Arc::try_unwrap(page_information_map)
        .expect("Arc still has multiple owners")
        .into_inner()
        .expect("Mutex cannot be locked");

    Ok((folder_tokens, page_information_map))
}

/// Read all urls in the given path to the urls.txt file. 
/// 
/// Returns a vector of all urls in the file, or an error if file did not exist
pub fn read_urls_file(file_path: &str) -> Result<Vec<String>, ()> {
    let mut vec: Vec<String> = Vec::new();
    let file = read_file(file_path);
    if let Ok(content) = file {
        for line in content.lines() {
            let tokens: Vec<&str> = line.split(";").into_iter().collect();
            if tokens.len() != 2 {
                return Err(());
            }
            vec.push(tokens[0].to_string())
        }

        return Ok(vec);
    }
    Err(())
}

/// Creates the paragraph text under the search result
///
/// Uses the document content that is already parsed from the html parser
/// NOTE: This is not working that well. It is very slow.
/// Need another way to get the paragraph from
pub fn get_key_word_paragraph(document_path:&str, terms: &String) -> String {
    let content = parse_file_html(document_path).expect(&format!("File not found {}", document_path));
    let mut paragraph = String::new();
    let terms_list: Vec<&str> = terms.split_whitespace().collect();
    let mut paragraph_lines: usize = 0;

    for line in content.lines() {
        let mut found = false;
        let mut highlighted_line = String::new();

        // Split the line into words and check each word
        for word in line.split_whitespace() {
            if terms_list.contains(&word) {
                found = true;
                highlighted_line.push_str(&format!("<strong>{}</strong> ", word));
            } else {
                highlighted_line.push_str(&format!("{} ", word));
            }
        }

        // If any term is found in the line, add the highlighted line to the paragraph
        if found && paragraph_lines < MAX_PARAGRAPH_LINES_COUNTER {
            paragraph.push_str(&highlighted_line.trim_end());
            paragraph.push('\n');
            paragraph_lines += 1;
        }
    }

    paragraph
}
