use crate::scanner::parser::{
    parse_attribute, parse_quote, parse_surrounding_tag, parse_tag_content,
};
use crate::scanner::result::ParsingResult;
use reqwest::{Client, Error};

pub async fn process_research(
    req_client: &Client,
    already_found_results: &Vec<ParsingResult>,
    base_url: &&str,
    url: &String,
    keywords_to_scan_in_pages: &Vec<&str>,
) -> Result<Vec<ParsingResult>, Error> {
    let search_page_content = req_client.get(url).send().await?.text().await?;
    let mut parsing_results: Vec<ParsingResult> = vec![];
    let main_content = parse_tag_content(&search_page_content, "main");
    if main_content.is_some() {
        let cards_urls = search_for_cards_urls(&main_content.unwrap(), base_url).await;
        for url in cards_urls {
            let page_content = req_client.get(&url).send().await?.text().await?;
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
    Ok(parsing_results)
}

pub async fn search_for_cards_urls(cards_list_content: &&str, base_url: &&str) -> Vec<String> {
    let mut urls: Vec<String> = vec![];
    let mut content_to_parse = *cards_list_content;
    while let Some(class_name_index) = content_to_parse.find("fr-card--horizontal") {
        let a_tag = parse_surrounding_tag(&content_to_parse, &class_name_index);
        if a_tag.is_some() {
            let o_href = parse_attribute(&a_tag.unwrap(), "href");
            if o_href.is_some() {
                urls.push((*base_url).to_owned() + o_href.unwrap());
            }
            content_to_parse = &content_to_parse[class_name_index + 10..]
        }
    }
    urls
}

pub async fn process_scan_page(
    url: &String,
    page_content: &String,
    keyword: &&str,
) -> Option<ParsingResult> {
    let main_content = parse_tag_content(&page_content, "main")?;
    let found_keyword_index = main_content.find(keyword)?;
    let quote = parse_quote(main_content, &found_keyword_index);

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
        quote,
    ))
}

fn parse_for_dates(page_content: &String) -> Option<(String, String)> {
    None

    // let main_content = parse_tag_content(page_content, "main")?;
    //
    // let set = RegexSet::new(&DATES_EXPRESSIONS).unwrap();
    // set.matches(main_content);


    // if dates.len() == 2 {
    //     Some((
    //         String::from(dates[0].clone()),
    //         String::from(dates[1].clone()),
    //     ))
    // } else {
    //     None
    // }
}
