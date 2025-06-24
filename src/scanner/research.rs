use reqwest::Client;

pub fn process_research (req_client: &Client, url: &str, research_keyword: &str, keywords_to_search_in_pages: &str) -> Option<String> {
    Some(String::from(url.to_owned() + "test"))
}