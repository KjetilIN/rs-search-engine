use std::collections::HashMap;

use crate::page_information::Website;

pub type TokenizedDocument = HashMap<String, usize>; 
pub type FolderTokens = HashMap<String, TokenizedDocument>; 
pub type PageInformationMap = HashMap<String, Website>; 