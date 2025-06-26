// REQUEST FORMAT = https://www.oise.gouv.fr/contenu/recherche?SearchText=consultation

mod utils;
mod scanner;

use crate::scanner::research::process_research;
use crate::scanner::result::ScanResult;
use crate::utils::set_panic_hook;
use reqwest::Client;
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

    let mut scan_results: Vec<ScanResult> = vec![];
    let research_keywords: Vec<&str> = research_keywords.split(",").collect();
    let keywords_to_scan_in_pages: Vec<&str> = keywords_to_scan_in_pages.split(",").collect();

    for research_keyword in research_keywords {
        let url = String::from(base_url.to_owned() + "/contenu/recherche?SearchText=" + research_keyword);
        process_research(&req_client, &mut scan_results, &url, &keywords_to_scan_in_pages).await;
    }

    String::from(base_url.to_owned() + "test")
}
