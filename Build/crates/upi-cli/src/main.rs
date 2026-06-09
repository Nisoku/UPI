use clap::{Parser, Subcommand};
use upi_core::Resolver;

#[derive(Parser)]
#[command(name = "upi", version, about = "Universal Package Installer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    Install { package: String },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Install { package } => run_install(package, cli.dry_run),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run_install(package: &str, dry_run: bool) -> Result<(), upi_core::Error> {
    let resolver = Resolver::new()?;
    let cmd = resolver.resolve(package)?;

    if dry_run {
        println!("{}", cmd.to_display());
    } else {
        cmd.run()?;
    }

    Ok(())
}
