fn parse_tag<'a>(content: &'a String, tag: &str) -> Option<&'a str> {
    let tag_start = format!("<{}", tag);
    let tag_start_index = content.find(tag_start.as_str())?;

    let tag_end = format!("</{}>", tag);
    let tag_end_index = content[tag_start_index..].find(tag_end.as_str())? - 1; // -1 -> "<" of </tag>

    Some(&content[tag_start_index..tag_end_index])
}

pub fn parse_tag_content<'a>(content: &'a String, tag: &str) -> Option<&'a str> {
    let parsed_tag = parse_tag(content, tag)?;
    let tag_start_closing = parsed_tag.find('>')? + 1; // +1 -> ">" of <tag ... >
    Some(&parsed_tag[tag_start_closing..])
}

pub fn parse_attribute<'a>(content: &str, attr: &str) -> Option<&'a str> {
    let attribute_index = content.find(attr)?;
    let attribute_content_start = attribute_index + attr.len() + 2; // +2 -> =" of attr="..."
    let attribute_content_closing = content[attribute_content_start..].find("\"")? - 1; // -1 -> the found "
    Some(&content[attribute_content_start..attribute_content_closing])
}
