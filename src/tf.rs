use crate::{
    page_information::Website,
    types::{FolderTokens, PageInformationMap, TokenizedDocument},
};
use std::{collections::HashMap, error::Error};

fn count_words(document: &TokenizedDocument) -> usize {
    let mut counter: usize = 0;
    for (_, value) in document.into_iter() {
        counter += value;
    }
    counter
}

fn document_with_term(term: &String, folder: &FolderTokens) -> usize {
    let mut counter: usize = 0;
    for (_, document_map) in folder.iter() {
        if document_map.contains_key(term) {
            counter += 1;
        }
    }
    counter
}

fn term_frequency(term: &String, document: &TokenizedDocument) -> f64 {
    if let Some(&word_count) = document.get(term) {
        let total_word_count = count_words(document);
        return word_count as f64 / total_word_count as f64;
    }
    0.0
}

fn inverse_document_frequency(term: &String, folder: &FolderTokens) -> f64 {
    let n = folder.len();
    let df = document_with_term(term, folder);
    if df > 0 {
        (n as f64 / df as f64).log10()
    } else {
        0.0
    }
}

fn tf_idf(term: String, document: &TokenizedDocument, folder: &FolderTokens) -> f64 {
    return term_frequency(&term, document) * inverse_document_frequency(&term, folder);
}

pub fn search_term(
    terms: &String,
    folder: &FolderTokens,
    page_information: &PageInformationMap,
) -> Result<Vec<Website>, Box<dyn Error>> {
    let search_terms: Vec<&str> = terms.split_whitespace().collect();
    let mut scores: HashMap<&String, f64> = HashMap::new();

    // Calculate TF-IDF score for each term in each document
    for term in &search_terms {
        for (doc_name, doc_tokens) in folder.iter() {
            let score = tf_idf(term.to_string(), doc_tokens, folder);
            *scores.entry(doc_name).or_insert(0.0) += score;
        }
    }

    // Convert the HashMap to a Vec and sort by TF-IDF scores in descending order
    let mut sorted_scores: Vec<(&String, f64)> = scores.into_iter().collect();
    sorted_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Retrieve the corresponding Website information
    let mut results: Vec<Website> = vec![];
    for (doc_name, score) in sorted_scores {
        if let Some(mut website) = page_information.get(doc_name).cloned() {
            website.tf_idf_score = score;
            results.push(website);
        }
    }

    Ok(results)
}
