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
    let tag_re = regex::Regex::new(r"<[^>]*>").unwrap();
    let weird_tag = regex::Regex::new(r"&[^;]*;").unwrap();
    weird_tag
        .replace_all(&tag_re.replace_all(&text.replace("<br />", ""), ""), "")
        .trim()
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

/// File checkings
/// Will check
/// - if the file exists
/// - if the file is a file
/// - if the file is readable
/// - if the file is empty
pub fn check_file(file_name: &str, file: &std::path::Path) -> PResult<()> {
    log::debug!("Checking file: {}", file.display());
    if !file.exists() {
        log::error!("File: {} does not exist.", file.display());
        return Err(crate::errors::Error::NotFound(
            file_name.to_owned(),
            file.display().to_string(),
        ));
    }
    if !file.is_file() {
        log::error!("File: {} is not a file.", file.display());
        return Err(crate::errors::Error::NotAFile(
            file_name.to_owned(),
            file.display().to_string(),
        ));
    }
    if std::fs::File::open(file).is_err() {
        log::error!("File: {} is not readable.", file.display());
        return Err(crate::errors::Error::NotReadable(
            file_name.to_owned(),
            file.display().to_string(),
        ));
    }
    if file.metadata()?.len() == 0 {
        log::error!("File: {} is empty.", file.display());
        return Err(crate::errors::Error::EmptyFile(
            file_name.to_owned(),
            file.display().to_string(),
        ));
    }
    log::debug!("File: {} is ok.", file.display());
    Ok(())
}
