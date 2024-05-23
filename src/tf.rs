use crate::types::{FolderTokens, TokenizedDocument};

fn count_words(document: &TokenizedDocument) -> usize{
    let mut counter:usize = 0; 
    for (_, value) in document.into_iter(){
        counter += value;
    }
    counter
}

fn count_documents(folder: &FolderTokens) -> usize{
    let mut counter:usize = 0; 
    for (_, _) in folder.into_iter(){
        counter += 1;
    }
    counter
}

fn document_with_term(term: &String, folder: &FolderTokens) -> usize{
    let mut counter: usize = 0;
    for (_, document_map) in folder.into_iter(){
        if document_map.contains_key(term){
            counter += 1;
        }
    }
    counter
}

fn term_frequency(term: &String, document: &TokenizedDocument) -> f64{
    if document.contains_key(term){
        let word_count = document.get(term).unwrap();
        let total_word_count = count_words(&document);
        return (word_count / total_word_count) as f64; 
    }else{
        0.0
    }
}

fn inverse_document_frequency(term: &String, folder: &FolderTokens) -> f64{
    let n = count_documents(folder);
    let tf = document_with_term(term, folder);
    ((n / (tf/n)) as f64).log10()
}

fn tf_idf(term: String, document: &TokenizedDocument, folder: &FolderTokens) -> f64{
    return term_frequency(&term, document) * inverse_document_frequency(&term, folder);
}