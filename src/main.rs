use anyhow::Result;
use clap::{Parser, Args, Subcommand};
use cmd::{adr::{adr_handler, AdrCommands}, config::{config_handler, ConfigCommands}, project::{project_handler, ProjectCommands}};
use log::info;

mod cmd;
mod utils;
mod adr;



#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
struct ProjectArgs {
    /// Directory to initialize
    #[arg(default_value = "doc/adr")]
    source: String,
    #[command(subcommand)]
    command: Option<ProjectCommands>,
}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
struct ConfigArgs {
    /// Directory to initialize
    #[arg(default_value = "doc/adr")]
    source: String,
    #[command(subcommand)]
    command: Option<ConfigCommands>,
}


#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true, version, about, long_about = None)]
struct AdrArgs {
    #[arg(default_value = "doc/adr")]
    source: String,
    #[command(subcommand)]
    command: Option<AdrCommands>,
}


#[derive(Subcommand)]
enum RootCommands {
    #[command(arg_required_else_help = true)]
    Adr(AdrArgs),
    #[command(arg_required_else_help = true)]
    Project(ProjectArgs),
    #[command(arg_required_else_help = true)]
    Config(ConfigArgs),
}


#[derive(Parser)]
#[command(version, about, long_about = None )]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: RootCommands,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    // let res = confy::get_configuration_file_path("adr-rs", None)?;
    // println!("the frigging path {:?}", res);
    let mut cfg: utils::config::AppConfig = confy::load("adr-rs", None)?;
    let cli = Cli::parse();
    match &cli.command {
        RootCommands::Adr(args) => {
            adr_handler(&mut cfg, args)?;
        }
        RootCommands::Project(args) => {
            project_handler(&mut cfg, args)?;
        },
        RootCommands::Config(args) => {
            config_handler(&mut cfg, args)?;
        },
    }

    Ok(())
}
