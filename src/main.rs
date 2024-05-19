use std::{collections::HashMap, fmt::Error, fs::{self, File}, io::{BufReader, Read}, process::exit};

type TokenizedDocument = HashMap<String, usize>; 
type FolderTokens = HashMap<String, TokenizedDocument>; 

const FOLDER_PATH: &str = "./pages/";


fn parse_file_html(file_path: &str) -> Result<TokenizedDocument, Error>{
    // Create new map for the tokens
    let mut map: TokenizedDocument = HashMap::new();

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

    // Iterate over each file 
    for line in content.lines(){
        let mut line_to_tokenize: String = line.to_string();
        let mut open_tag_index: i32 = -1; 

        // Iterate over each char in the line and 
        for (index, char) in line.chars().enumerate(){
            if char == '<'{
                open_tag_index = index as i32;
                continue;
            }else if char == '>' {
                if open_tag_index > -1 {
                    // Tag can be removed, and reset start index
                    line_to_tokenize.replace_range(open_tag_index as usize..index, "");
                    open_tag_index = -1; 
                }else{
                    eprintln!("[ERROR] Found '>' without a opening tag in '{line}': {file_path}");
                    exit(1)
                }
                
            }
        }

        // Check if there is an opening tag without closing tag
        if open_tag_index != -1{
            eprintln!("[ERROR] Found '<' without a closing tag in {file_path}");
            exit(1)
        }

        // Collect tokens 
        let tokens: Vec<&str> = line_to_tokenize.split(" ").into_iter().collect();   
    }

    Ok(map)
}


fn parse_file_md(file_path: &str) -> Result<Option<TokenizedDocument>, Error>{
    unimplemented!("Parse Markdown File")
}

fn parse_dir(folder_path: &str) -> Result<FolderTokens, ()> {
    let mut folder_tokens = HashMap::new();

    let paths = fs::read_dir(folder_path).unwrap_or_else(|err| {
        eprintln!("[ERROR] Was not able to read dir path {folder_path}:{err}");
        exit(1)
    });

    for path in paths{
        if let Some(current_path) = path.unwrap().path().to_str(){
            println!("[INFO] Parsing file {}", current_path);
            _ = parse_file_html(current_path);
        }else{
            eprintln!("[ERROR] File not available");
            exit(1)
        }
    }
    
    Ok(folder_tokens)
}

fn main() {
    
    _ = parse_dir(FOLDER_PATH);

}
