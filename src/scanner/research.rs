use crate::scanner::result::ScanResult;
use reqwest::Client;

pub async fn process_research(req_client: &Client, scan_results: &mut Vec<ScanResult>, base_url: &&str, url: &String, keywords_to_scan_in_pages: &Vec<&str>) {
    let search_page_content = req_client.get(url).send().await.unwrap().text().await.unwrap();
    let cards_list_content = get_cards_list_from_page_content(&search_page_content);

    let cards_urls = parse_for_cards_urls(&cards_list_content, base_url).await;
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

pub fn get_cards_list_from_page_content(search_page_content: &String) -> &str {
    let start_ul_index = search_page_content.find("<ul>").unwrap_or(0);
    let end_ul_index = search_page_content.rfind("</ul>").unwrap_or(0);
    &search_page_content[start_ul_index..end_ul_index]
}

pub async fn parse_for_cards_urls(cards_list_content: &&str, base_url: &&str) -> Vec<String> {
    let mut urls: Vec<String> = vec![];
    // first, split the string between elements that encapsulate the href of cards
    let parts_with_href = cards_list_content.split("fr-card__title").collect::<Vec<&str>>().iter().flat_map(|part| part.split("</a>")).collect::<Vec<&str>>().into_iter().filter(|link| link.contains("fr-card__link")).collect::<Vec<&str>>();
    for part_to_trim in parts_with_href {
        // then, reduce the string to <href="my-url.com">
        let first_index = part_to_trim.find("href").unwrap_or(0) + 6; // 6 = href="
        let second_index = part_to_trim.rfind("\"").unwrap_or(0) - 1; // 1 = "
        // finally, trim the ends nicely to only keep the url
        let url = &part_to_trim[first_index..second_index];
        urls.push((*base_url).to_owned() + url);
    }
    urls
}

pub async fn process_scan_page(url: &String, _keyword: &&str) -> Option<ScanResult> {
    Some(ScanResult::new(url.clone(), String::from("07/11/2018"), String::from("jamais"), String::from("test")))
}