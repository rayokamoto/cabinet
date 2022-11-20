use clap::Command;

mod commands;
mod path;

const NAME: &str = "Cabinet";
const BIN_NAME: &str = "cab";
const VERSION: &str = "0.4.0";
const ABOUT: &str = "A convenient file sorting utility";

// TODO: Add option for config file to automate things more
// TODO: Add option to sort folders as well as files
fn main() {
    let about_text = format!("{} {}\n{}", NAME, VERSION, ABOUT);
    let usage_text = format!("{} <command> [options] [<path>]", BIN_NAME);
    let after_help_text = format!("See '{} help <command>' for more information on a command", BIN_NAME);

    let cabinet = Command::new("cabinet")
        .name(NAME)
        .version(VERSION)
        .about(about_text)
        .bin_name(BIN_NAME)
        .arg_required_else_help(true)
        .override_usage(usage_text)
        .after_help(after_help_text)
        .subcommands(commands::builtin());

    let matches = cabinet.get_matches();

    match matches.subcommand() {
        Some(("date", cmd)) => {
            commands::date::exec(cmd);
        }
        Some(("name", cmd)) => {
            commands::name::exec(cmd);
        }
        Some(("size", cmd)) => {
            commands::size::exec(cmd);
        }
        Some(("type", cmd)) => {
            commands::file_type::exec(cmd);
        }
        _ => {
            unreachable!();
        }
    }
}
