use clap::Parser;

mod bot;
mod cli;
mod config;
mod errors;
mod utils;

async fn try_main() -> errors::Result<()> {
    // Skip the first argument, which is the program name.
    pretty_env_logger::init();
    let cli = cli::Cli::parse();
    cli.check()?;
    log::debug!("CLI arguments: {:#?}", cli);
    println!("Running the bot. Press Ctrl+C to stop.");
    bot::run(cli).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
