use std::ffi::OsString;
use std::env::args_os;

mod commands;
use commands::help;

mod parser;
use parser::{TokenType, Token};
mod path;


const DEBUG: bool = false;

/// Parse the command line arguments and tokenize them accordingly
fn parse_args() -> Vec<Token> {
    let mut argv: Vec<OsString> = args_os().collect();
    argv.remove(0); // Remove the filename from the array
    let argc = argv.len();
    //println!("Arg length: {}; Arguments: {:?}", argc, argv);

    // Construct list of tokens
    let mut arg_list: Vec<Token> = Vec::new();

    // If there are no arguments
    if argc == 0 {
        let tok = Token {
            of_type: TokenType::SubCommand,
            value: String::from("help"),
            position: 0
        };
        arg_list.push(tok);
        return arg_list;
    }

    // For the first subcommand/argument (very important)
    let arg = argv[0].to_str().unwrap().to_string();
    if arg.starts_with("--") {
        let tok = Token {
            of_type: TokenType::Argument,
            value: argv[0].to_str().unwrap().to_string(),
            position: 0
        };
        arg_list.push(tok);
    }
    else if arg.starts_with("-") {
        let tok = Token {
            of_type: TokenType::Argument,
            value: argv[0].to_str().unwrap().to_string(),
            position: 0
        };
        arg_list.push(tok);
    }
    // Otherwise, assume that it is a subcommand
    else {
        let tok = Token {
            of_type: TokenType::SubCommand,
            value: argv[0].to_str().unwrap().to_string(),
            position: 0
        };
        arg_list.push(tok);
    }

    argv.remove(0); // Remove (base) subcommand/argument



    for (pos, arg) in argv.iter().enumerate() {
        let arg = arg.to_str().unwrap().to_string();

        // Currently does not support use of equals sign for arguments that take values
        // TODO: accept equal sign in arguments
        // TODO: maybe do this at the subcommand level?
        if arg.starts_with("--") {
            let tok = Token {
                of_type: TokenType::Argument,
                value: arg,
                position: 0
            };
            arg_list.push(tok);
        }
        else if arg.starts_with("-") {
            let tok = Token {
                of_type: TokenType::Argument,
                value: arg,
                position: 0
            };
            arg_list.push(tok);
        }
        else {
            let tok = Token {
                of_type: TokenType::ArgumentValue,
                value: arg,
                position: pos+1
            };
            arg_list.push(tok);
        }
    }

    return arg_list;
}


/// Check command line arguments and decide whether it is a flag/option, help command or other subcommand
fn check_args(arg_list: &Vec<Token>, cmd: &String) {
    let argc = arg_list.len();
    if argc >= 2 {
        if ["-h", "--help", "help"].contains(&&arg_list[1].value[..]) {
            // run specific help command
            println!("Specific help command for `{}`", cmd);
            help::cmd_help(cmd);
            return;
        }
        let cmd = &cmd[..];
        match cmd {
            "date" => commands::file_date::file_date(&arg_list),
            "name" => commands::file_name::file_name(&arg_list),
            "size" => commands::file_size::file_size(&arg_list),
            "type" => commands::file_type::file_type(&arg_list),
            _ => println!("ERROR")
        }
    }
    else { // only the subcommand itself was invoked.
        help::cmd_help(cmd);
    }
}



// TODO: Keep improving command line parser
// TODO: address use of process::exit()
// TODO: Add option for config file to automate things more
// TODO: Add option to sort folders as well as files
fn main() {
    // TODO: (maybe) Add option to break up an option like "-rf" and separate into "-r" and "-f" for parsing purposes
    let arg_list = parse_args();
    let subcommand_list = commands::get_subcommands();

    if DEBUG {
        println!("{:?}", arg_list);
        for (pos, arg) in arg_list.iter().enumerate() {
            println!("Pos: {};  Arg: \"{}\"", pos, arg.value);
        }
    }


    // arg_list will contain at least one argument.
    let arg0 = &arg_list[0];
    if ["-h", "--help", "help"].contains(&&arg0.value[..]) {
        help::help();
    }

    else if arg0.of_type == TokenType::Argument {
        println!("ERROR: The flag \"{}\" does not exist.", arg0.value);
    }
    else { // Token type will be subcommand by default
        if subcommand_list.contains(&arg0.value) {
            check_args(&arg_list, &arg0.value)
        }
        else {
            println!("The command \"{}\" does not exist.", arg0.value);
        }
    }

}
