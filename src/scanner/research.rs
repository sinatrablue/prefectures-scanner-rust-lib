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
    let cards_list_content = get_cards_list_from_page_content(&search_page_content);

    let cards_urls = parse_for_cards_urls(&cards_list_content, base_url).await;
    let mut parsing_results: Vec<ParsingResult> = vec![];
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
                Some(result) => {
                    if !already_found_results
                        .iter()
                        .any(|parsing_result| parsing_result.is_same_url(&result.url))
                    {
                        parsing_results.push(result);
                    }
                }
                None => {}
            }
        }
    }
    Ok(parsing_results)
}

pub fn get_cards_list_from_page_content(search_page_content: &String) -> &str {
    let start_ul_index = search_page_content.find("<ul>").unwrap_or(0);
    let end_ul_index = search_page_content.rfind("</ul>").unwrap_or(0);
    &search_page_content[start_ul_index..end_ul_index]
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
        // then, reduce the string to <href="my-url.com">
        let first_index = part_to_trim.find("href").unwrap_or(0) + 6; // 6 = href="
        let second_index = part_to_trim.rfind("\"").unwrap_or(0) - 1; // 1 = "
        // finally, trim the ends nicely to only keep the url
        let url = &part_to_trim[first_index..second_index];
        urls.push((*base_url).to_owned() + url);
    }
    urls
}

pub async fn process_scan_page(
    url: &String,
    page_content: &String,
    keyword: &&str,
) -> Option<ParsingResult> {
    let found_keyword_index = page_content.find(keyword);
    let quote: &str;
    match found_keyword_index {
        Some(index) => {
            quote = &page_content[index - 20..index + 25];
        }
        None => {
            return None;
        }
    }

    let found_h1_index = page_content.find("<h1");
    let title: String;

    match found_h1_index {
        Some(opening_h1_start_index) => {
            let closing_h1_index = page_content
                .find("</h1>")
                .unwrap_or(opening_h1_start_index + 25);

            let opening_h1_end_index = page_content[opening_h1_start_index..closing_h1_index]
                .find(">")
                .unwrap_or(0)
                + 1
                + opening_h1_start_index;

            // TODO: some sort of findNextLiteral
            title = page_content[opening_h1_end_index..closing_h1_index].to_owned();
        }
        None => {
            title = "Titre non identifié".to_owned();
        }
    }

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
        title,
        quote.to_string(),
    ))
}

fn parse_for_dates(page_content: &String) -> Option<(String, String)> {
    let main_start_index = page_content.find("<main>")?;
    let main_end_index = page_content.rfind("</main>")?;
    let main_content = &page_content[main_start_index..main_end_index];

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
