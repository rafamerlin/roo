use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[clap(
        short = 'f',
        long,
        value_parser,
        help = "Simple filter results by text"
    )]
    pub filter: Option<String>,

    #[clap(value_parser, help = "Command key to use based on config file")]
    pub command: Option<String>,

    #[clap(value_parser, default_value_t = String::from("."), help = "The directory to run as root when searching (optional)")]
    pub path: String,

    #[clap(subcommand)]
    pub sub_command: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    #[clap(about = "Shows the current loaded configurations")]
    Config,
}

pub fn get_args() -> Args {
    Args::parse()
}
