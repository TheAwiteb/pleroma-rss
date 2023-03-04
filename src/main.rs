mod bot;
mod cli;
mod errors;
mod utils;

async fn try_main() -> errors::Result<()> {
    // Skip the first argument, which is the program name.
    let cli = cli::Cli::parse(std::env::args().skip(1).collect())?;
    if cli.help {
        println!("{}", cli::help_message());
    } else if cli.version {
        println!("{}", cli::version_message());
    } else {
        bot::run(cli).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
