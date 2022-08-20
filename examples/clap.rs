//This is more to play around with Clap in case I need to add more arguments/subcommands later on.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[clap(
        short = 'f',
        long,
        value_parser,
        help = "Simple filter results by text"
    )]
    filter: Option<String>,

    #[clap(value_parser, help = "Command key to use based on config file")]
    command: Option<String>,

    #[clap(value_parser, default_value_t = String::from("."), help = "The directory to run as root when searching (optional)")]
    path: String,

    #[clap(subcommand)]
    sub_command: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Config(ConfigCommands),
    #[clap(help = "Shows the Test loaded configurations")]
    Test,
}

#[derive(clap::Args, Debug)]
struct ConfigCommands {
    #[clap(long, short)]
    list: bool,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    //  // You can check for the existence of subcommands, and if found use their
    // // matches just as you would the top level cmd
    // match &args.sub_command {
    //     SubCommands::Config { cfg } => {
    //         println!("'myapp Config' was used, value is: {:?}", cfg)
    //     }
    // }
}
