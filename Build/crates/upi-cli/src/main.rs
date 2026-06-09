use clap::{Parser, Subcommand};
use upi_core::{PackageSource, PlatformRegistry, Resolver};
use upi_net::RepologyClient;

#[derive(Parser)]
#[command(name = "upi", version, about = "Universal Package Installer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true)]
    dry_run: bool,

    #[arg(long, global = true, help = "Skip network lookups")]
    offline: bool,
}

#[derive(Subcommand)]
enum Commands {
    Install { package: String },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Install { package } => run_install(package, cli.dry_run, cli.offline),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run_install(
    package: &str,
    dry_run: bool,
    offline: bool,
) -> Result<(), upi_core::Error> {
    let registry = PlatformRegistry::load()?;

    let sources: Vec<Box<dyn PackageSource>> = if offline {
        Vec::new()
    } else {
        let client = RepologyClient::new(registry.clone())
            .map_err(|e| upi_core::Error::Network(format!("repology: {e}")))?;
        vec![Box::new(client)]
    };

    let resolver = Resolver::with_registry_and_sources(registry, sources)?;
    let cmd = resolver.resolve(package)?;

    if dry_run {
        println!("{}", cmd.to_display());
    } else {
        cmd.run()?;
    }

    Ok(())
}
