use clap::{crate_version, App, AppSettings, Arg, SubCommand};

pub fn build_app() -> App<'static, 'static> {
    App::new("roo")
        .version(crate_version!())
        .usage("roo [FLAGS/OPTIONS] [<pattern>] [<path>]")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("filter")
                .required(false)
                .help("Simple filter results by text")
                .short("f")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("command")
                .required(false)
                .help("Command key to use based on config file")
                .long_help(""),
        )
        .arg(
            Arg::with_name("path")
                .required(false)
                .help("the root directory for the filesystem search (optional)")
                .long_help(
                    "The directory where the filesystem search is rooted (optional). If \
                         omitted, search the current working directory.",
                ),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Used for configuration")
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("lists the configuration"),
                )
                .arg(
                    Arg::with_name("add")
                        .short("a")
                        .long("add")
                        .help("add new configuration"),
                )
                .arg(
                    Arg::with_name("import")
                        .short("i")
                        .long("import")
                        .help("import config file")
                        .value_name("path"),
                ),
        )
}
