pub fn parse_tag<'a>(content: &'a String, tag: &str) -> Option<&'a str> {
    let tag_start = format!("<{}", tag);
    let tag_start_index = content.find(tag_start.as_str())?;

    let tag_end = format!("</{}>", tag);
    let tag_end_index = tag_start_index + content[tag_start_index..].find(tag_end.as_str())? + tag_end.len();

    Some(&content[tag_start_index..tag_end_index])
}

pub fn parse_tag_content<'a>(content: &'a String, tag: &str) -> Option<&'a str> {
    let parsed_tag = parse_tag(content, tag)?;
    let tag_start_closing = parsed_tag.find('>')? + 1; // +1 -> ">" of <tag ... >
    let tag_end = format!("</{}>", tag);
    let tag_end_index = parsed_tag.len() - tag_end.len();
    Some(&parsed_tag[tag_start_closing..tag_end_index])
}

pub fn parse_attribute<'a>(content: &'a str, attr: &str) -> Option<&'a str> {
    let attribute_index = content.find(attr)?;
    let attribute_content_start = attribute_index + attr.len() + 2; // +2 -> =" of attr="..."
    let attribute_content_closing = attribute_content_start + content[attribute_content_start..].find("\"")?;
    Some(&content[attribute_content_start..attribute_content_closing])
}

pub fn parse_surrounding_tag<'a>(content: &'a str, found_index: &usize) -> Option<&'a str> {
    let tag_start_closing_index = content[..*found_index].rfind(">")? + *found_index;
    let tag_start_opening_index = content[..tag_start_closing_index].rfind("<")? + *found_index;
    let tag = content[tag_start_opening_index..tag_start_closing_index];
    let start_index = found_index + tag_start_opening_index;
    let end_index = content[*found_index..].find("</")? + tag.len() + 1 + *found_index;
    Some(&content[start_index..end_index])
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
        assert_eq!(parsed_tag_content, "
                                Espèces et habitats protégés
                           ")
    }

    #[test]
    fn it_parses_an_attribute() {
        let href_content = "/Actions-de-l-Etat/Environnement/Nature-et-Biodiversite/Especes-et-habitats-proteges";
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
}