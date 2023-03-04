mod cli;
mod errors;

fn try_main() -> errors::Result<()> {
    // Skip the first argument, which is the program name.
    let cli = cli::Cli::parse(std::env::args().skip(1).collect())?;
    if cli.help {
        println!("{}", cli::help_message());
    } else if cli.version {
        println!("{}", cli::version_message());
    }
    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
