use std::io::BufRead;

use reqwest::Url;

use crate::bot::Feed;
use crate::errors::Result as PResult;

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

/// Parses the RSS feeds file. It returns a list of feeds.
/// ### File format
/// The file must contain one feed url per line.
/// The url must be valid.
/// ```text
/// https://example.com/feed
/// https://example.com/feed2
/// ```
pub fn parse_feeds(rss_feeds_file: &std::path::Path, only_new: bool) -> PResult<Vec<Feed>> {
    log::debug!("Opening feeds file...");
    let file = std::fs::File::open(rss_feeds_file)?;
    let reader = std::io::BufReader::new(file);
    log::debug!("Reading feeds file...");
    reader
        .lines()
        .filter(|line| !line.as_ref().unwrap().is_empty())
        .map(|line| {
            let line = line?;
            log::debug!("Parsing feed: {}, only_new: {}", line, only_new);
            Ok(Feed::new(Url::parse(&line)?, only_new))
        })
        .collect()
}
