use std::{error::Error, fs::File, io::{BufReader, Read}};

use crate::types::FolderTokens;

const CACHE_FILE_PATH: &str = "./cache/hashmap_cache.dat";

pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>>{
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

pub fn save_to_file(folder:FolderTokens) -> Result<(), Box<dyn Error>>{
    let file = match File::create(CACHE_FILE_PATH){
        Ok(f) => f,
        Err(err) => {
            eprintln!("[ERROR] Could not create file: {err}");
            return Err(Box::new(err));
        },
    };

    match bincode::serialize_into(file, &folder){
        Ok(_) => return Ok(()),
        Err(err) => {
            eprintln!("[ERROR] Could write to file: {err}");
            return Err(Box::new(err));
        },
    };
}

pub fn load_from_file() -> Result<FolderTokens, Box<dyn Error>>{
    let file = match File::open(CACHE_FILE_PATH){
        Ok(f) => f,
        Err(err) => {
            eprintln!("[ERROR] Could not create file: {err}");
            return Err(Box::new(err));
        },
    };

    let folder: FolderTokens = match bincode::deserialize_from(file) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("[ERROR] Could not read from file: {err}");
            return Err(Box::new(err));
        },
    };

    Ok(folder)
}