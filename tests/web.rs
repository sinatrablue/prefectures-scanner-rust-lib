//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate prefectures_scanner_rust_lib;
extern crate wasm_bindgen_test;

use prefectures_scanner_rust_lib::scan_prefecture;
use prefectures_scanner_rust_lib::scanner::parser::parse_tag_content;
use prefectures_scanner_rust_lib::scanner::research::{process_scan_page, search_for_cards_urls};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_scan() {
    let url = "https://www.oise.gouv.fr";
    let research_keywords = "[consult]";
    let in_page_keywords = "[consult,public,concertation,examen]";
    let results = scan_prefecture(url, research_keywords, in_page_keywords).await;
    assert_eq!(results, "test");
}

#[wasm_bindgen_test]
async fn test_cards_urls_scan() {
    let search_page_content =
        reqwest::get("https://www.oise.gouv.fr/contenu/recherche?SearchText=consultation")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    let main_content = parse_tag_content(&search_page_content, "main").unwrap();
    let res = search_for_cards_urls(&main_content, &"https://www.oise.gouv.fr").await;
    assert_eq!(res.len(), 10);
}

#[wasm_bindgen_test]
async fn test_page_scan() {
    let url = String::from("https://www.oise.gouv.fr/Actions-de-l-Etat/Environnement/Chasse-et-Peche/La-chasse-et-la-faune-sauvage/Consultation-du-public-sur-la-chasse/Consultation-en-cours/Projet-arrete-mini-maxi-cervides");
    let page_content = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let keyword = "consultation";
    let res = process_scan_page(&url, &page_content, &keyword).await;
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.url, url);
    // assert_eq!(res.start_date, String::from("11 juin"));
    // assert_eq!(res.end_date, String::from("2 juillet"));
    assert_eq!(
        res.title,
        String::from("Projet arrêté mini maxi cervidés - consultation close")
    );
}
