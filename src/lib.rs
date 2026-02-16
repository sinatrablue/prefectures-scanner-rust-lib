// REQUEST FORMAT = https://www.oise.gouv.fr/contenu/recherche?SearchText=consultation

mod utils;
pub mod scanner;

use crate::scanner::errors::ParsingError;
use crate::scanner::research::process_research;
use crate::scanner::result::{ParsingResult, ScanResult};
use crate::utils::set_panic_hook;
use reqwest::Client;
use serde_json::json;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn scan_prefecture(base_url: &str, research_keywords: &str, keywords_to_scan_in_pages: &str) -> String {
    set_panic_hook();

    let req_client = Client::new();

    let mut parsing_results: Vec<ParsingResult> = vec![];
    let mut parsing_errors: Vec<ParsingError> = vec![];
    let research_keywords: Vec<&str> = research_keywords.split(",").collect();
    let keywords_to_scan_in_pages: Vec<&str> = keywords_to_scan_in_pages.split(",").collect();

    for research_keyword in research_keywords {
        let url = String::from(base_url.to_owned() + "/contenu/recherche?SearchText=" + research_keyword);
        match process_research(&req_client, &parsing_results, &base_url, &url, &keywords_to_scan_in_pages).await {
            Ok(mut result) => parsing_results.append(&mut result),
            Err(error) => parsing_errors.push(ParsingError::new(url, error)),
        }
    }

    let scan_results = ScanResult::new(String::from(base_url), parsing_results, parsing_errors);
    json!(scan_results).to_string()
}
