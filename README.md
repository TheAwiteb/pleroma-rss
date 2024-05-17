<div align="center">

**Deprecation Notice: This project is no longer maintained.**

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
Usage: pleroma-rss [OPTIONS] --access-token <TOKEN> --feeds-file <PATH> --base-url <URL>

Options:
  -a, --access-token <TOKEN>   Your bot access token
  -f, --feeds-file <PATH>      The file that contains the feeds
  -b, --base-url <URL>         The server URL
  -s, --items-sleep <SECONDS>  The sleep time between each feed in seconds [default: 1]
  -w, --watting-new <SECONDS>  The sleep time after end all feeds (wait for new items) in seconds [default: 30]
  -m, --mastodon               Use Mastodon instead of Pleroma
  -n, --only-new               Only post new items. Without this flag, the bot will post all the items in the feed
  -d, --dry-run                Do not post anything, will print the items that would be posted
  -h, --help                   Print help
  -V, --version                Print version
```
### Example
```bash
$ pleroma-rss -b https://bassam.social -a 1234567890 -f feeds.txt
```

### Mastodon Support
We also support Mastodon instances. You can use the `--mastodon` flag to enable Mastodon support

### `preview-image` feature
> Disabled by default

The `preview-image` feature will create a preview image for each feed item. Will get the image from the `media:content` tag if it exists, otherwise it will use `--default-preview-image`. The image will deleted after it is posted.
#### Enabling the feature
To enable the feature, you need to compile the program with the `preview-image` feature. You can do that by running the following command:
```bash
$ cargo install -F preview-image --locked --git https://github.com/TheAwiteb/pleroma-rss
```
#### Extra flags
There is tow extra flags when using the `preview-image` feature, `--default-preview-image` and `--preview-image-template`. The `--default-preview-image` flag is used to set the default image that will be used if the feed item doesn't have an image. The `--preview-image-template` flag is used to set the template for the preview image. The template is a HTML file can use any CSS style. The template can use the following variables:
- `{{title}}`: The title of the feed item
- `{{description}}`: The description of the feed item (first **320** characters full words)
- `{{link}}`: The link of the feed item
- `{{image-src}}`: The source of the image
#### Extra dependencies
The `preview-image` feature requires the wkhtmltoimage binary to be installed on your system. You can download it from [here](https://wkhtmltopdf.org/downloads.html). The binary must be in your `$PATH`. You can check if it is installed by running the following command:
```bash
$ wkhtmltoimage --version
wkhtmltoimage 0.12.6 (with patched qt)
```
#### Example
```bash
$ pleroma-rss -b https://bassam.social -a 1234567890 -f feeds.txt --default-preview-image default.png --preview-image-template template.html
```
#### Example output
<img src="https://i.suar.me/jdZEx/" width="300" alt="Example output">

## Feeds file
The feeds file is a simple text file with one feed url per line. The file can be located anywhere on your system. The path to the file is passed to the program using the `-f` or `--feed-file` flag.
### Example
```bash
$ cat feeds.txt
https://example.com/feed.xml
https://example.com/feed2.xml
```

### Example output
<img src="https://i.suar.me/17Yzw/" width="300" alt="Example output">

## Logging
To enable logging, set the `RUST_LOG` environment variable to `info` or `debug`. For example:
```bash
$ RUST_LOG=debug pleroma-rss -b https://bassam.social -a 1234567890 -f feeds.txt
```

## Contributing
Pull requests are welcome. Please open an issue first to discuss what you would like to change.

## License
This project is licensed under the AGPL-3.0 License - see the [LICENSE](https://github.com/TheAwiteb/pleroma-rss/blob/master/LICENSE) file for details
