mod color;

use std::io::{IsTerminal, Write};
use std::time::Duration;

use clap::{ArgAction, Parser, Subcommand};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use log::LevelFilter;
use upi_core::{detect, OsType, PackageSource, PlatformRegistry, Resolver};
use upi_net::RepologyClient;

#[derive(Parser)]
#[command(
    name = "upi",
    version,
    about = "Universal Package Installer",
    before_help = ""
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true, help = "Show command without executing")]
    dry_run: bool,

    #[arg(long, global = true, help = "Skip network lookups")]
    offline: bool,

    #[arg(
        long,
        global = true,
        help = "Override target OS (e.g., macos, debian, arch)"
    )]
    os: Option<String>,

    #[arg(
        long,
        global = true,
        help = "Allow installing the raw query as-is when no confident match is found"
    )]
    allow_identity: bool,

    #[arg(
        short,
        long,
        global = true,
        action = ArgAction::Count,
        help = "Increase verbosity (-v, -vv, -vvv)"
    )]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package
    Install { package: String },
    /// Resolve a package name to an install command
    Search { package: String },
}

fn main() {
    print_banner();
    let cli = Cli::parse();
    init_logger(cli.verbose);

    let result = match &cli.command {
        Commands::Install { package } => run(package, &cli),
        Commands::Search { package } => run_search(package, &cli),
    };

    if let Err(e) = result {
        finish_spinner(ProgressBar::new_spinner(), String::new());
        match e {
            upi_core::Error::Resolve(msg) => {
                if let Some((main, hint)) = msg.split_once("Re-run with ") {
                    eprintln!("  {} {}", color::red("error:"), main);
                    eprintln!("         Re-run with {hint}");
                } else {
                    eprintln!("  {} {msg}", color::red("error:"));
                }
            }
            other => {
                eprintln!("  {} {other}", color::red("error:"));
            }
        }
        std::process::exit(1);
    }
}

fn init_logger(verbosity: u8) {
    let level = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    env_logger::Builder::new()
        .filter_level(level)
        .parse_env("RUST_LOG")
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
}

fn resolve_os(registry: &PlatformRegistry, os_override: &Option<String>) -> OsType {
    os_override
        .as_deref()
        .and_then(|name| registry.parse_os(name).cloned())
        .unwrap_or_else(detect)
}

fn build_sources(
    registry: &PlatformRegistry,
    offline: bool,
) -> Result<Vec<Box<dyn PackageSource>>, upi_core::Error> {
    if offline {
        Ok(Vec::new())
    } else {
        let client = RepologyClient::new(registry.clone())
            .map_err(|e| upi_core::Error::Network(format!("repology: {e}")))?;
        Ok(vec![Box::new(client)])
    }
}

fn is_interactive() -> bool {
    std::env::var("CI").is_err() && std::io::stderr().is_terminal()
}

fn print_banner() {
    let art = [
        "██╗   ██╗██████╗ ██╗",
        "██║   ██║██╔══██╗██║",
        "██║   ██║██████╔╝██║",
        "██║   ██║██╔═══╝ ██║",
        "╚██████╔╝██║     ██║",
        " ╚═════╝ ╚═╝     ╚═╝",
    ];
    eprintln!("{}", color::green(&art.join("\n")));
}

fn spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    if !is_interactive() {
        pb.set_draw_target(ProgressDrawTarget::hidden());
    }
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("◐◓◑◒"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

fn finish_spinner(pb: ProgressBar, msg: String) {
    pb.set_style(ProgressStyle::with_template("{msg}").unwrap());
    pb.finish_with_message(msg);
}

fn run(package: &str, cli: &Cli) -> Result<(), upi_core::Error> {
    let spinner = spinner();
    spinner.set_message(format!("resolving {package}"));
    let registry = PlatformRegistry::global();

    let os_type = resolve_os(registry, &cli.os);
    let sources = build_sources(registry, cli.offline)?;
    let resolver = Resolver::with_registry_and_sources(registry.clone(), sources)?
        .allow_identity(cli.allow_identity);

    let commands = resolver.resolve_commands_for_os(package, &os_type)?;

    if cli.dry_run {
        let msg = format!(
            "{} {}",
            color::green("✔"),
            color::bold(&format!("resolved {package}"))
        );
        finish_spinner(spinner, msg);
        if let Some(cmd) = commands.first() {
            println!();
            println!("  {}", color::bright_green(&cmd.to_display()));
            println!();
        }
    } else {
        let mut last_error = None;
        for cmd in commands {
            match cmd.run() {
                Ok(()) => {
                    let msg = format!(
                        "{} {}",
                        color::green("✔"),
                        color::bold(&format!("installed {package}"))
                    );
                    finish_spinner(spinner, msg);
                    return Ok(());
                }
                Err(upi_core::Error::ProgramNotFound(_)) => {
                    last_error = Some(upi_core::Error::Resolve(format!(
                        "package manager not available for {os_type:?}"
                    )));
                    continue;
                }
                Err(err) => return Err(err),
            }
        }

        return Err(last_error.unwrap_or_else(|| {
            upi_core::Error::Resolve(format!("no install command available for {os_type:?}"))
        }));
    }

    Ok(())
}

fn run_search(package: &str, cli: &Cli) -> Result<(), upi_core::Error> {
    let spinner = spinner();
    spinner.set_message(format!("searching for {package}"));
    let registry = PlatformRegistry::global();

    let os_type = resolve_os(registry, &cli.os);
    let sources = build_sources(registry, cli.offline)?;

    let (manager, config_clone) = {
        let c = registry.for_type(&os_type);
        (c.map(|c| c.manager.clone()), c.cloned())
    };

    let resolver = Resolver::with_registry_and_sources(registry.clone(), sources)?;
    let candidates = resolver.search_candidates(package, &os_type)?;

    let msg = format!(
        "{} {}",
        color::green("✔"),
        color::bold(&format!("searched for {package}"))
    );
    finish_spinner(spinner, msg);

    println!();
    println!("  {}       {os_type:?}", color::green("OS:"));
    println!(
        "  {}  {}",
        color::green("Manager:"),
        manager.as_deref().unwrap_or("?")
    );
    println!("  {}    {package}", color::green("Query:"));
    println!();
    println!("  {}", color::green("Results:"));
    for c in &candidates {
        let source_label = if c.source.starts_with("database") {
            color::green(&c.source)
        } else if c.source == "identity" {
            color::dim(&c.source)
        } else {
            c.source.clone()
        };
        let padded = format!("{:<30}", c.name);
        println!("    {}  <- {}", color::bold(&padded), source_label);
    }
    if let Some(ref cfg) = config_clone {
        let primary = candidates
            .first()
            .map(|c| c.name.as_str())
            .unwrap_or(package);
        let cmd = upi_core::Command::from_config(cfg, primary);
        println!();
        println!(
            "  {}  {}",
            color::green("Command:"),
            color::bright_green(&cmd.to_display())
        );
    }
    println!();

    Ok(())
}
