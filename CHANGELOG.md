# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.4.0 - 2023-03-07
See [0.4.0-rc.1](#0.4.0-rc.1) for the changes in this release candidate
### Added
- Use `clap`, now the CLI looks better [[600366b](https://github.com/TheAwiteb/pleroma-rss/commit/600366bf683fd902346eefed9456b2a52562f0cc)]
- Add a `--items-sleep` flag to specify the sleep time between each feed in seconds [[PR #5](https://github.com/TheAwiteb/pleroma-rss/pull/5)]
- Add a `--watting-new` flag to specify the sleep time after end all feeds (wait for new items) in seconds [[PR #5](https://github.com/TheAwiteb/pleroma-rss/pull/5)]
- Now the bot supports mastodon instances, you can add the `--mastodon` flag to use it with mastodon instances [[PR #6](https://github.com/TheAwiteb/pleroma-rss/pull/6)]

### Changed
- Update the help message, now it looks better
- Improve the error messages

## 0.4.0-rc.1 - 2023-03-06
### Added
- Now there is a `preview-image` feature that creates a preview image for the feed item
- Add a `--preview-image-template` flag to specify a template for the preview image (`preview-image` feature)
- Add a `--default-preview-image` flag to specify a default image when the feed item does not have an image (`preview-image` feature)

## [0.3.0] - 2023-03-05
### Added
- Add a `--only-new` flag to only post new feed items
- Add a `--dry-run` flag to only print the feed items that would be posted


## [0.2.0] - 2023-03-04
### Added
- The bot now can be run, readd the feeds file and post the feed items to the pleroma instance.
- Enable logging using the `RUST_LOG` environment variabl
- Check if the feed file exists and is readable.
- Check if the feed file is not empty.
- Check if the feed file is not a directory.

### Changed
- The help message is now printed if no arguments are passed to the program. [[`07f2bef`](https://github.com/TheAwiteb/pleroma-rss/commit/07f2beff4f38a24c972e6c8ef38cfd178a0e4539)]
### Removed
- Remove the `--verbose` flag. [[`8e3d85c`](https://github.com/TheAwiteb/pleroma-rss/commit/8e3d85c714a2f43da0f887d8b217f6ff6a3f08dc)]

## [0.1.0] - 2023-03-04
### Added
- CLI parsing for:
  - Flags:
    - `-a`, `--access--token` The access token of the bot account
    - `-b`, `--base--url`     The base url of the pleroma instance
    - `-f`, `--feed--file`    The path to the feeds file 
  - Options:
    - `-h`, `--help`          Prints help information
    - `-V`, `--version`       Prints version information
    - `-v`, `--verbose`       Prints verbose information
