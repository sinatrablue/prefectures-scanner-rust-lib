//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate prefectures_scanner_rust_lib;
extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/*
#[wasm_bindgen_test]
async fn test_scan() {
    let url = "https://www.oise.gouv.fr";
    let research_keywords = "[consultation]";
    let in_page_keywords = "[consultation]";
    let results = scan_prefecture(url, research_keywords, in_page_keywords).await;
    assert_eq!(results.len(), 7);
}
*/

/*
#[wasm_bindgen_test]
async fn test_cards_urls_scan() {
    let search_page_content = reqwest::get("https://www.oise.gouv.fr/contenu/recherche?SearchText=consultation").await.unwrap().text().await.unwrap();
    let cards_list_content = get_cards_list_from_page_content(&search_page_content);
    let res = parse_for_cards_urls(&cards_list_content, &"https://www.oise.gouv.fr").await;
    assert_eq!(res.len(), 10);
}
*/

/*
#[wasm_bindgen_test]
async fn test_page_scan() {
    let url = String::from("https://www.oise.gouv.fr/Actions-de-l-Etat/Environnement/Chasse-et-Peche/La-chasse-et-la-faune-sauvage/Consultation-du-public-sur-la-chasse/Consultation-en-cours/Projet-arrete-mini-maxi-cervides");
    let keyword = "consultation";
    let res = process_scan_page(&url, &keyword).await;
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.url, url);
    assert_eq!(res.start_date, String::from("11 juin"));
    assert_eq!(res.end_date, String::from("2 juillet"));
    assert_eq!(res.text, String::from("Projet arrêté mini maxi cervidés"));
}
*/
