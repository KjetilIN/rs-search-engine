use types::{FolderTokens, TokenizedDocument};

mod parse;
pub mod types;
pub mod tf;

fn main() {
    let folder_path: &str = "./pages/";
    let documents: FolderTokens = parse::parse_dir(folder_path, true, true).unwrap();

    println!("FOLDER: {:?}", documents);

}
