use clap::{ Command, Subcommand};
use std::{error::Error, process::exit};

use crate::{api::serve_website, file_operations::save_to_file, types::FolderTokens};


pub mod parse;
pub mod api;
pub mod file_operations;
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

const FOLDER_PATH: &str = "./pages/";

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("RS Engine")
        .version("1.0")
        .about("Search engine CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("serve")
                .about("Run the webserver in the browser"),
        )
        .subcommand(
            Command::new("parse")
                .about("Parse HTML files for the search engine")
                .subcommand(
                    Command::new("file")
                        .about("Save to file(s)")
                )
                .subcommand(
                    Command::new("db")
                        .about("Save to database")
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("serve", _)) => {
            println!("[INFO] Starting webserver...");
            serve_website();
        },
        Some(("parse", sub_m)) => {
            match sub_m.subcommand() {
                Some(("file", _)) => {
                    println!("[INFO] Parsing to file...");
                    let (documents, page_information ) = parse::parse_dir(FOLDER_PATH, true, true).unwrap();

                    println!("{:?}", page_information);

                    match save_to_file("tokens.dat".to_owned(),documents){
                        Ok(_) => println!("[INFO] Saved tokens.dat to file!"),
                        Err(_) => exit(1),
                    }

                    match save_to_file("page_lookup.dat".to_owned(),page_information){
                        Ok(_) => println!("[INFO] Saved page_lookup.dat to file!"),
                        Err(_) => exit(1),
                    }

                },
                Some(("db", _)) => {
                    println!("[ERROR] Database not implemented yet");
                },
                _ => eprintln!("[ERROR] Unknown parse command"),
            }
        },
        _ => eprintln!("[ERROR] Unknown command"),
    }

    Ok(())

    
}
