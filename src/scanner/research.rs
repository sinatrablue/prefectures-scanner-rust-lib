use crate::scanner::result::ScanResult;
use reqwest::Client;

pub async fn process_research(req_client: &Client, scan_results: &mut Vec<ScanResult>, url: &String, keywords_to_scan_in_pages: &Vec<&str>) {
    let search_page_content = req_client.get(url).send().await.unwrap().text().await.unwrap();

    let cards_urls = parse_for_cards_urls(&search_page_content).await;
    for url in cards_urls {
        for keyword in keywords_to_scan_in_pages {
            let page_scan_result = process_scan_page(&url, keyword).await;
            match page_scan_result {
                Some(result) => {
                    if !scan_results.iter().any(|scan_result| scan_result.is_same_url(&result.url)) {
                        scan_results.push(result);
                    }
                }
                None => {}
            }
        }
    }
}

async fn parse_for_cards_urls(page_content: &String) -> Vec<String> {
    vec![String::from(page_content)]
}

async fn process_scan_page(url: &String, _keyword: &&str) -> Option<ScanResult> {
    Some(ScanResult::new(url.clone(), String::from("07/11/2018"), String::from("test")))
}