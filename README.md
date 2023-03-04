<div align="center">

# Pleroma RSS
A Pleroma bot that sends RSS feeds to your followers. keep your followers up to date with your favorite RSS feeds.

[![Continuous Integration](https://github.com/TheAwiteb/pleroma-rss/actions/workflows/ci.yml/badge.svg)](https://github.com/TheAwiteb/pleroma-rss/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/TheAwiteb/pleroma-rss)](https://github.com/TheAwiteb/pleroma-rss/blob/master/LICENSE)

</div>

## Installation
You can install the binary from the [releases page](https://github.com/TheAwiteb/pleroma-rss/releases/latest) or useing [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) with the following command:
```bash
cargo install --locked --git https://github.com/TheAwiteb/pleroma-rss
```
The binary will be installed in `~/.cargo/bin/pleroma-rss`. Make sure that this directory is in your `$PATH`.
### Build from source
```bash
git clone https://github.com/TheAwiteb/pleroma-rss
cd pleroma-rss
cargo build --release
```
The binary will be in `target/release/pleroma-rss`

## Usage
```bash
$ pleroma-rss --help
USAGE:
    pleroma-rss [FLAGS] [OPTIONS]
FLAGS:
    -h, --help    Prints help information
    -V, --version Prints version information
OPTIONS:
    -a, --access-token <access-token> The access token of the bot account
    -b, --base-url <base-url>         The base url of the pleroma instance
    -f, --feed-file <feed-file>       The path to the feeds file 
```
### Example
```bash
$ pleroma-rss -b https://pleroma.site -a 1234567890 -f feeds.txt
```
## Feeds file
The feeds file is a simple text file with one feed url per line. The file can be located anywhere on your system. The path to the file is passed to the program using the `-f` or `--feed-file` flag.
### Example
```bash
$ cat feeds.txt
https://example.com/feed.xml
https://example.com/feed2.xml
```

## License
This project is licensed under the AGPL-3.0 License - see the [LICENSE](https://github.com/TheAwiteb/pleroma-rss/blob/master/LICENSE) file for details
