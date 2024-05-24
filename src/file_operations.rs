use std::{error::Error, fs::File, io::{BufReader, Read}};

use serde::{Deserialize, Serialize};
const CACHE_FILE_PATH: &str = "./cache/";

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

pub fn save_to_file<T: Serialize>(file_name:String, data:T) -> Result<(), Box<dyn Error>>{
    let path = format!("{}{}", CACHE_FILE_PATH, file_name);
    let file = match File::create(path){
        Ok(f) => f,
        Err(err) => {
            eprintln!("[ERROR] Could not create file: {err}");
            return Err(Box::new(err));
        },
    };

    match bincode::serialize_into(file, &data){
        Ok(_) => return Ok(()),
        Err(err) => {
            eprintln!("[ERROR] Could write to file: {err}");
            return Err(Box::new(err));
        },
    };
}

pub fn load_from_file<T: for<'de> Deserialize<'de>>(file_name: String) -> Result<T, Box<dyn Error>> {
    let path = format!("{}{}", CACHE_FILE_PATH, file_name);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("[ERROR] Could not open file: {err}");
            return Err(Box::new(err));
        },
    };

    let data: T = match bincode::deserialize_from(file) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("[ERROR] Could not read from file: {err}");
            return Err(Box::new(err));
        },
    };

    Ok(data)
}