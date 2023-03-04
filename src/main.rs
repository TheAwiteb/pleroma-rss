mod bot;
mod cli;
mod errors;
mod utils;

async fn try_main() -> errors::Result<()> {
    // Skip the first argument, which is the program name.
    pretty_env_logger::init();
    let cli = cli::Cli::parse(std::env::args().skip(1).collect())?;
    log::info!("The cli args: {:?}", cli);
    if cli.help {
        log::info!("Printing help message.");
        println!("{}", cli::help_message());
    } else if cli.version {
        log::info!("Printing version message.");
        println!("{}", cli::version_message());
    } else {
        println!("Running the bot. Press Ctrl+C to stop.");
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
