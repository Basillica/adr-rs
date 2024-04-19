use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;

mod cmd;
mod utils;
mod adr;


#[derive(Parser)]
#[command(version, about, long_about = None )]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(cmd::init::InitArgs),
    Export(cmd::export::ExportArgs),
    Import(cmd::import::ImportArgs),
    List(cmd::list::ListArgs),
    New(cmd::new::NewArgs),
    Unlink(cmd::unlink::UnlinkArgs),
    Link(cmd::link::LinkArgs),
    Update(cmd::update::UpdateArgs),
    Config(cmd::config::ConfigArgs),
}

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    let mut cfg: utils::config::AppConfig = confy::load("adr-rs", None)?;
    println!("{:#?}", cfg);

    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(args) => {
            cmd::init::main(args, &mut cfg)?;
        }
        Commands::New(args) => {
            cmd::new::main(args, &mut cfg)?;
        },
        Commands::Import(args) => {
            cmd::import::main(args)?;
        },
        Commands::Export(args) => {
            cmd::export::main(args)?;
        },
        Commands::Link(args) => {
            cmd::link::main(args)?;
        },
        Commands::Unlink(args) => {
            cmd::unlink::main(args)?;
        },
        Commands::List(args) => {
            cmd::list::main(args, &mut cfg)?;
        },
        Commands::Update(args) => {
            cmd::update::main(args, &mut cfg)?;
        },
        Commands::Config(args) => {
            cmd::config::main(args)?;
        },
    }

    Ok(())
}
