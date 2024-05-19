use std::{collections::HashMap, fmt::Error, fs::{self, File}, io::{BufReader, Read}, process::exit};

type TokenizedDocument = HashMap<String, usize>; 
type FolderTokens = HashMap<String, TokenizedDocument>; 

const FOLDER_PATH: &str = "./pages/";


fn parse_file_html(file_path: &str) -> Result<TokenizedDocument, Error>{
    let mut map: HashMap<_, _> = HashMap::new();

    let mut file =  match File::open(file_path){
        Ok(file) => file,
        Err(e) => {
            eprintln!("[ERROR] Was not able to find file {}: {}", file_path, e);
            panic!()
        }
    };

    let mut content = String::new();
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content).unwrap_or_else(|err| {
        eprintln!("[ERROR] could not read file {}: {}", file_path, err);
        panic!()
    });

    for line in content.lines(){
        println!("{}", line);
    }

    Ok(map)
}


fn parse_file_md(file_path: &str) -> Result<Option<TokenizedDocument>, Error>{
    unimplemented!("Parse Markdown File")
}

fn parse_folder(folder_path: &str) -> Result<FolderTokens, ()> {
    let mut folder_tokens = HashMap::new();

    let paths = fs::read_dir(folder_path).unwrap();

    for path in paths{
        if let Some(current_path) = path.unwrap().path().to_str(){
            println!("[INFO] Parsing file {}", current_path);
        }else{
            eprintln!("[ERROR] File not available");
            exit(1)
        }
    }
    
    Ok(folder_tokens)
}

fn main() {
    
    _ = parse_folder(FOLDER_PATH);

}
