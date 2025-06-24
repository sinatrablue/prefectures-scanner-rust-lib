//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use prefectures_scanner_rust_lib::scan_prefecture;

extern crate prefectures_scanner_rust_lib;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_scan() {
    let url = "https://www.oise.gouv.fr";
    let research_keywords = "[consultation, public]";
    let in_page_keywords = "[consultation]";
    let returned_url = scan_prefecture(url, research_keywords, in_page_keywords);
    assert_eq!(returned_url, url.to_owned() + "test");
}
