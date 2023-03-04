/// Rmove html tags from a string
/// useing regex
pub fn remove_html_tags(text: &str) -> String {
    let re = regex::Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(&text.replace("<br />", "").replace("&quot;", ""), "")
        .to_string()
}
