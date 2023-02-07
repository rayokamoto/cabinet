use std::env;

use clap::{Arg, Command};

mod commands;
mod sort;
mod util;

const NAME: &str = "Cabinet";
const BIN_NAME: &str = "cab";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const ABOUT: &str = "A convenient file sorting utility";

// TODO: Add option for config file (e.g. add your own template paths)
// TODO: Add option to sort folders as well as files
// TODO: Better handling of symlinks
fn main() {
    let about_text = format!("{} {}\n{}", NAME, VERSION, ABOUT);
    let usage_text = format!("{} <command> [options] [<path>]", BIN_NAME);
    let after_help_text = format!(
        "See '{} help <command>' for more information on a command",
        BIN_NAME
    );

    let cabinet = Command::new("cabinet")
        .name(NAME)
        .version(VERSION)
        .about(about_text)
        .bin_name(BIN_NAME)
        .arg_required_else_help(true)
        .override_usage(usage_text)
        .after_help(after_help_text)
        .subcommands(commands::builtin())
        .args([Arg::new("output")
            .long("output")
            .short('o')
            .help("Specify the name of the output folder")
            .action(clap::ArgAction::Set)
            .global(true)]);

    let matches = cabinet.get_matches();

    match matches.subcommand() {
        Some(("date", cmd)) => commands::date::exec(cmd),
        Some(("multisort", cmd)) => commands::multisort::exec(cmd),
        Some(("name", cmd)) => commands::name::exec(cmd),
        Some(("size", cmd)) => commands::size::exec(cmd),
        Some(("type", cmd)) => commands::file_type::exec(cmd),
        _ => unreachable!(),
    }
}
