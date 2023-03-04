/// Rmove html tags from a string
/// useing regex
pub fn remove_html_tags(text: &str) -> String {
    log::info!(
        "Removing html tags from: {}...",
        text.chars().take(10).collect::<String>()
    );
    let re = regex::Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(&text.replace("<br />", "").replace("&quot;", ""), "")
        .to_string()
}
