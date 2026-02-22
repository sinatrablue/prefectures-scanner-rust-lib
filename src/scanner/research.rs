use crate::scanner::parser::{parse_attribute, parse_tag_content};
use crate::scanner::{constants::DATES_EXPRESSIONS, result::ParsingResult};
use regex::RegexSet;
use reqwest::{Client, Error};

pub async fn process_research(
    req_client: &Client,
    already_found_results: &Vec<ParsingResult>,
    base_url: &&str,
    url: &String,
    keywords_to_scan_in_pages: &Vec<&str>,
) -> Result<Vec<ParsingResult>, Error> {
    let search_page_content = req_client
        .get(url)
        .send()
        .await?
        .text()
        .await?;
    let mut parsing_results: Vec<ParsingResult> = vec![];

    let cards_list_content = parse_tag_content(&search_page_content, "ul");
    match cards_list_content {
        None => {}
        Some(cards_list_content) => {
            let cards_urls = parse_for_cards_urls(&cards_list_content, base_url).await;
            for url in cards_urls {
                let page_content = req_client
                    .get(&url)
                    .send()
                    .await?
                    .text()
                    .await?;
                for keyword in keywords_to_scan_in_pages {
                    let page_scan_result = process_scan_page(&url, &page_content, keyword).await;
                    match page_scan_result {
                        None => {}
                        Some(result) => {
                            if !already_found_results
                                .iter()
                                .any(|parsing_result| parsing_result.is_same_url(&result.url))
                            {
                                parsing_results.push(result);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(parsing_results)
}

pub async fn parse_for_cards_urls(cards_list_content: &&str, base_url: &&str) -> Vec<String> {
    let mut urls: Vec<String> = vec![];
    // first, split the string between elements that encapsulate the href of cards
    let parts_with_href = cards_list_content
        .split("fr-card__title")
        .collect::<Vec<&str>>()
        .iter()
        .flat_map(|part| part.split("</a>"))
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|link| link.contains("fr-card__link"))
        .collect::<Vec<&str>>();
    for part_to_trim in parts_with_href {
        let url = parse_attribute(part_to_trim, "href");
        match url {
            None => {}
            Some(url) => {
                urls.push((*base_url).to_owned() + url);
            }
        }
    }
    urls
}

pub async fn process_scan_page(
    url: &String,
    page_content: &String,
    keyword: &&str,
) -> Option<ParsingResult> {
    let found_keyword_index = page_content.find(keyword)?;
    let quote = &page_content[found_keyword_index - 20..found_keyword_index + 25];

    let title = parse_tag_content(page_content, "h1").unwrap_or("Titre non identifié");

    let (start_date, end_date) = match parse_for_dates(page_content) {
        Some((found_start_date, found_end_date)) => (found_start_date, found_end_date),
        None => (
            String::from("Date non trouvée"),
            String::from("Date non trouvée"),
        ),
    };

    Some(ParsingResult::new(
        url.clone(),
        start_date,
        end_date,
        title.to_owned(),
        quote.to_string(),
    ))
}

fn parse_for_dates(page_content: &String) -> Option<(String, String)> {
    let main_content = parse_tag_content(page_content, "main")?;

    let set = RegexSet::new(&DATES_EXPRESSIONS).unwrap();
    set.matches(main_content);

    return None;

    // if dates.len() == 2 {
    //     Some((
    //         String::from(dates[0].clone()),
    //         String::from(dates[1].clone()),
    //     ))
    // } else {
    //     None
    // }
}
