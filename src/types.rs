use std::collections::HashMap;

pub type TokenizedDocument = HashMap<String, usize>; 
pub type FolderTokens = HashMap<String, TokenizedDocument>; 