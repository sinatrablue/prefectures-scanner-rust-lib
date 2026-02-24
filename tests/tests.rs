#[cfg(test)]
mod tests {
    use prefectures_scanner_rust_lib::scanner::parser::parse_tag;

    #[test]
    fn test_parse_tag() {
        let some_html_card_body = String::from("<div class=\"fr-card__body\">
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
                </div>");
        let some_a_tag = String::from("<a href=\"/Actions-de-l-Etat/Environnement/Nature-et-Biodiversite/Especes-et-habitats-proteges\" class=\"fr-card__link\">
                                Espèces et habitats protégés
                           </a>");

        let parsed_tag = parse_tag(&some_html_card_body, "a");
        assert_eq!(parsed_tag.unwrap(), some_a_tag);
    }
}