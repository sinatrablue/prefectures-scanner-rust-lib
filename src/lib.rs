// REQUEST FORMAT = https://www.oise.gouv.fr/contenu/recherche?SearchText=consultation

mod utils;
mod scanner;

use crate::scanner::research::process_research;
use crate::utils::set_panic_hook;
use reqwest::Client;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn scan_prefecture(url: &str, research_keywords: &str, keywords_to_search_in_pages: &str) -> String {
    set_panic_hook();

    let req_client = Client::new();

    let mut scan_results = vec![];
    for research_keyword in research_keywords.split(',') {
        let res = process_research(&req_client, url, research_keyword, keywords_to_search_in_pages);
        match res {
            Some(res) => scan_results.push(res),
            None => {}
        }
    }

    scan_results[0].clone()
}
