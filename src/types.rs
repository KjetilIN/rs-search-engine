use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Website{
    url: String, 
    title: String
}

const KUBERNETES_URL: &str = "https://kubernetes.io"; 

impl Website {
    pub fn from_html(content: &str, file_path: &str) -> Self{

        let url = if file_path.ends_with("_index.html") {
            // Remove "index.html" and append the rest to the base URL
            let path = &file_path[..file_path.len() - "_index.html".len()];
            format!("{}{}", KUBERNETES_URL, &path[7..])
        } else if file_path.ends_with("index.html"){
            let path = &file_path[..file_path.len() - "index.html".len()];
            format!("{}{}", KUBERNETES_URL, &path[7..])

        }else {
            format!("{}{}", KUBERNETES_URL, &file_path[7..])
        };

        // Extract title from the content, assuming title is in the second line
        let raw_title = content.lines().nth(1).unwrap_or_default().to_string();

        // Remove "title: " prefix if present
        let title_prefix = "title: ";
        let title = if raw_title.to_lowercase().starts_with(title_prefix) {
            raw_title[title_prefix.len()..].to_string()
        } else {
            raw_title
        };

        // If title is still empty, extract the last folder name from the file path
        let title = if title.is_empty() {
            file_path
                .rsplit('/')
                .nth(1) // nth(1) to get the last folder name instead of the file name
                .unwrap_or("Default")
                .to_string()
        } else {
            title
        };

        Self { url, title }
    }
    
}

pub type TokenizedDocument = HashMap<String, usize>; 
pub type FolderTokens = HashMap<String, TokenizedDocument>; 
pub type PageInformationMap = HashMap<String, Website>; 