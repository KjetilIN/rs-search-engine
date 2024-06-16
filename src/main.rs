use clap::{Command, Subcommand};
use std::{error::Error, process::exit};

use crate::{
    api::serve_website,
    file_operations::{load_from_file, save_to_file},
    types::{FolderTokens, PageInformationMap},
};

pub mod api;
pub mod file_operations;
pub mod page_information;
pub mod parse;
pub mod tf;
pub mod types;

#[derive(Subcommand)]
enum Commands {
    Serve,
    Parse {
        #[clap(subcommand)]
        parse_cmd: ParseCommand,
    },
}

#[derive(Subcommand)]
enum ParseCommand {
    File,
    Db,
}

// Path to the folder with all the html files to be parsed 
const FOLDER_PATH: &str = "./pages/";

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("RS Engine")
        .version("1.0")
        .about("Search engine CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("serve").about("Run the webserver in the browser"))
        .subcommand(
            Command::new("parse")
                .about("Parse HTML files for the search engine")
                .subcommand(Command::new("file").about("Save to file(s)"))
                .subcommand(Command::new("db").about("Save to database")),
        )
        .subcommand(
            Command::new("load")
                .about("Loads the saved .dat files for the search engine and prints them"),
        )
        .get_matches();

    // Subcommands for the binary 
    match matches.subcommand() {
        // Host HTTP server 
        Some(("serve", _)) => {
            println!("[INFO] Starting webserver...");
            serve_website();
        }

        // Index to either file or db 
        Some(("index", sub_m)) => match sub_m.subcommand() {
            Some(("file", _)) => {
                println!("[INFO] Parsing to file...");
                let (documents, page_information) =
                    parse::parse_dir(FOLDER_PATH, true, true).unwrap();

                match save_to_file("tokens.dat".to_owned(), documents) {
                    Ok(_) => println!("[INFO] Saved tokens.dat to file!"),
                    Err(_) => exit(1),
                }

                match save_to_file("page_lookup.dat".to_owned(), page_information) {
                    Ok(_) => println!("[INFO] Saved page_lookup.dat to file!"),
                    Err(_) => exit(1),
                }
            }

            Some(("db", _)) => {
                println!("[ERROR] Database not implemented yet");
            }
            _ => eprintln!("[ERROR] Unknown parse command"),
        },
        
        // Check the indexed files by loading them and printing 
        Some(("load", _)) => {
            println!("[INFO] Loading tokens.dat");
            let tokens_folder: FolderTokens = match load_from_file("tokens.dat".to_string()) {
                Ok(val) => {
                    println!("[INFO] Successfully loaded tokens.dat");
                    val
                }
                Err(_) => {
                    eprintln!("[ERROR] Was not able to load tokens.dat..");
                    exit(1)
                }
            };

            let page_lookup: PageInformationMap =
                match load_from_file("page_lookup.dat".to_string()) {
                    Ok(val) => {
                        println!("[INFO] Successfully loaded tokens.dat");
                        val
                    }
                    Err(_) => {
                        eprintln!("[ERROR] Was not able to load tokens.dat..");
                        exit(1)
                    }
                };

            println!("\n\n Tokens: {:?}\n", tokens_folder);
            println!("\n\n Page Information: {:?}\n", page_lookup);
        }
        _ => eprintln!("[ERROR] Unknown command"),
    }

    Ok(())
}
