use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Website{
    url: String, 
    title: String,
    release: String,
    pub tf_idf_score: f64,
}

impl Website {
    pub fn from_html(content: &str, url:&str) -> Self{
        let mut title = String::new();
        let mut release: String = String::new();

        // Iterate trough the lines
        for line in content.lines(){
            if !release.is_empty(){
                break;
            }

            let tag_regex: Regex = Regex::new(r"<[^>]*>").unwrap();
            let line_to_tokenize: String = tag_regex.replace_all(&line, "").to_string();

            if let Some(new_title) = line_to_tokenize.trim().strip_prefix("Title:") {
                title = new_title.trim().to_string();
            } else if let Some(release_line) = line.trim().strip_prefix("Release date:") {
                release = release_line.chars().take_while(|&x| x != '[').collect();
            }
        }

        Self { 
            url:url.to_string(), 
            title,
            release, 
            tf_idf_score:0.0 
        }
    }
    
}