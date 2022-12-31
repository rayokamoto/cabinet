use clap::{Arg, ArgMatches, Command};

pub fn cli() -> Command {
    Command::new("multisort")
        .about("Sort files using multiple file attributes")
        .alias("sort")
        //.args([])
        .arg_required_else_help(true)
        .arg(
            Arg::new("path")
                .action(clap::ArgAction::Set)
                .value_name("PATH")
                .required(true),
        )
        .subcommand_value_name("PATH")
}

pub fn exec(args: &ArgMatches) {}
