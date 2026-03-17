pub fn parse_tag<'a>(content: &'a String, tag: &str) -> Option<&'a str> {
    let tag_start = format!("<{}", tag);
    let tag_start_index = content.find(tag_start.as_str())?;

    let tag_end = format!("</{}>", tag);
    let tag_end_index =
        tag_start_index + content[tag_start_index..].find(tag_end.as_str())? + tag_end.len();

    Some(&content[tag_start_index..tag_end_index])
}

pub fn parse_tag_content<'a>(content: &'a String, tag: &str) -> Option<&'a str> {
    let parsed_tag = parse_tag(content, tag)?;
    let tag_start_closing = parsed_tag.find('>')? + 1; // +1 -> ">" of <tag ... >
    let tag_end = format!("</{}>", tag);
    let tag_end_index = parsed_tag.len() - tag_end.len();
    let res = &parsed_tag[tag_start_closing..tag_end_index];
    Some(res.trim_matches(|c| c == ' ' || c == '\n'))
}

pub fn parse_attribute<'a>(content: &'a str, attr: &str) -> Option<&'a str> {
    let attribute_index = content.find(attr)?;
    let attribute_content_start = attribute_index + attr.len() + 2; // +2 -> =" of attr="..."
    let attribute_content_closing =
        attribute_content_start + content[attribute_content_start..].find("\"")?;
    Some(&content[attribute_content_start..attribute_content_closing])
}

pub fn parse_surrounding_tag<'a>(content: &'a str, found_index: &usize) -> Option<&'a str> {
    let i = *found_index;
    let tag_start_opening_index = content[..i].rfind("<")?;
    let tag_name_end = match content[tag_start_opening_index..i].find(" ") {
        Some(tag_name_end) => tag_name_end + tag_start_opening_index,
        None => content[tag_start_opening_index..i].find(">")? + tag_start_opening_index,
    };
    let tag = &content[tag_start_opening_index + 1..tag_name_end];
    let end_index = i + content[i..].find("</")? + tag.len() + 3; // 3 covers </> in </tag>
    Some(&content[tag_start_opening_index..end_index])
}

pub fn parse_quote(content: &str, found_index: &usize) -> String {
    let mut before_content = &content[..*found_index];
    if let Some(found_tag_pattern) = before_content.rfind(">") {
        before_content = &content[found_tag_pattern + 1..*found_index]; // exclude the found tag pattern
    }
    let mut before_chars = before_content.chars().collect::<Vec<_>>();

    let mut after_content = &content[*found_index..];
    if let Some(found_tag_pattern) = after_content.find("<") {
        after_content = &content[*found_index..*found_index + found_tag_pattern];
    }
    let after_chars = after_content.chars().collect::<Vec<_>>();

    let before_kept = if before_chars.len() > 40 {
        before_chars.reverse();
        let mut res = before_chars[0..40].to_vec();
        res.reverse();
        res
    } else {
        before_chars
    };
    let after_kept = if after_chars.len() > 50 {
        after_chars[0..50].to_vec()
    } else {
        after_chars
    };
    [before_kept, after_kept]
        .concat()
        .iter()
        .collect::<String>()
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    const SOME_HTML_CARD_BODY: &str = "<div class=\"fr-card__body\">
                    <div class=\"fr-card__content\">
                        <h3 class=\"fr-card__title\">
                            <a href=\"/Actions-de-l-Etat/Environnement/Nature-et-Biodiversite/Especes-et-habitats-proteges\" class=\"fr-card__link\">
                                Espèces et habitats protégés
                           </a>
                        </h3>
                        <p class=\"fr-card__desc\">, un premier avis sera formulé et la demande sera transmise pour avis consultatif au CNPN (Comité National de la Protection de la Nature) ou au CSRPN (...)</p>
                        <div class=\"fr-card__end\">
                            <p class=\"fr-card__detail\"><span>Mis à jour le 12/02/2026</span></p>
                        </div>
                    </div>
                </div>";

    const SOME_A_TAG: &str = "<a href=\"/Actions-de-l-Etat/Environnement/Nature-et-Biodiversite/Especes-et-habitats-proteges\" class=\"fr-card__link\">
                                Espèces et habitats protégés
                           </a>";

    #[test]
    fn it_parses_a_tag() {
        let content = SOME_HTML_CARD_BODY.to_string();
        let parsed_tag = parse_tag(&content, "a").unwrap();
        assert_eq!(parsed_tag, SOME_A_TAG);
    }

    #[test]
    fn it_parses_a_tag_content() {
        let content = SOME_HTML_CARD_BODY.to_string();
        let parsed_tag_content = parse_tag_content(&content, "a").unwrap();
        assert_eq!(parsed_tag_content, "Espèces et habitats protégés")
    }

    #[test]
    fn it_parses_an_attribute() {
        let href_content =
            "/Actions-de-l-Etat/Environnement/Nature-et-Biodiversite/Especes-et-habitats-proteges";
        let parsed_attribute = parse_attribute(SOME_A_TAG, "href").unwrap();
        assert_eq!(parsed_attribute, href_content);
    }

    #[test]
    fn it_parses_the_surrounding_tag() {
        let tag = "<p class=\"fr-card__desc\">, un premier avis sera formulé et la demande sera transmise pour avis consultatif au CNPN (Comité National de la Protection de la Nature) ou au CSRPN (...)</p>";
        let found_index = SOME_HTML_CARD_BODY.find("fr-card__desc").unwrap();
        let found_tag = parse_surrounding_tag(SOME_HTML_CARD_BODY, &found_index).unwrap();
        assert_eq!(found_tag, tag);
    }

    #[test]
    fn it_parses_a_quote() {
        let content = String::from("Un contenu avec le mot-clé que l'on recherche là ici c'est consultation donc on rajoute du texte et on met la fonction au défi d'extraire la quote");
        let found_index = content.find("consultation").unwrap();
        let quote = parse_quote(&content, &found_index);
        assert_eq!(quote, "mot-clé que l'on recherche là ici c'est consultation donc on rajoute du texte et on met la");
        let content = String::from("<p class=\"fr-card__desc\">La demande sera transmise pour avis consultatif au CNPN (Comité National)</p>");
        let found_index = content.find("consult").unwrap();
        let quote = parse_quote(&content, &found_index);
        assert_eq!(
            quote,
            "La demande sera transmise pour avis consultatif au CNPN (Comité National)"
        );
    }
}
