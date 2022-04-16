use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::DirEntry;
use std::env::args_os;
use std::path::{Path, PathBuf};

use std::{
    io::{stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};

mod commands;
use commands::help;

mod parser;
use parser::{TokenType, Token, SubCommand};
mod path;
use path::{get_path, ArgumentType};

const DEBUG: bool = false;


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

    let tok = Token {
        of_type: TokenType::SubCommand,
        value: argv[0].to_str().unwrap().to_string(),
        position: 0
    };
    arg_list.push(tok);
    argv.remove(0); // Remove (base) command


    for (pos, arg) in argv.iter().enumerate() {
        let arg = arg.to_str().unwrap().to_string();

        if arg.starts_with("-") {
            let tok = Token {
                of_type: TokenType::Argument,
                value: arg,
                position: pos+1
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


fn check_args(args: &Vec<Token>, command: SubCommand) -> (Option<PathBuf>, Option<Vec<Token>>) {
    // NOTE: repeated letters for option/flag does not change anything. It's like a set, whereby duplicates get removed.

    let mut has_opts: bool = false;
    let mut opts: Vec<Token> = Vec::new();

    let mut has_params: bool = false;
    let mut params: Vec<Token> = Vec::new();
    
    let mut path_exists: bool = false;
    let mut path_type = ArgumentType::None;

    // path type (path | template) is not incl. in opts
    for val in args {
        if val.of_type == TokenType::Argument {
            has_opts = true;
            if ["-p", "--path"].contains(&&val.value[..]) {
                path_type = ArgumentType::Path;
                path_exists = true;
            }
            else if ["-t", "--template"].contains(&&val.value[..]) {
                path_type =  ArgumentType::Template;
                path_exists = true;
            }
            else {
                opts.push(val.clone());
            }
        }
        if val.of_type == TokenType::ArgumentValue {
            has_params = true;
            params.push(val.clone());
        }
    }

    if has_opts && !has_params {
        if ["-h", "--help", "help"].contains(&&args[1].value[..]) {
            help::cmd_help(command);
        }
        else if ["-p", "--path"].contains(&&args[1].value[..]) || 
            ["-t", "--template"].contains(&&args[1].value[..]) {
                println!("Path must be provided.");
        }
        else {
            println!("Invalid operation.");
            help::cmd_help(command);
        }
    }
    else if !has_opts && !has_params {
        println!("Missing arguments.\n");
        help::cmd_help(command);
    }
    else if !has_opts && has_params {
        // options were not specified 
        println!("Options were not specified.");
    }
    else if path_exists && has_params {
        // TODO :Have a default behavior of absolute path being default
        let result = get_path(&params[0].value, path_type); // assume first param is path
        if result != None {
            // return the result (path) and any other remaining options/params
            return (result, Some(opts));
        }
        // if none, do nothing
    }
    else {
        // error: path decl not provided.
        println!("Path type specifier not provided. Use '-p' or '-t' flags to specify whether you want to use a path or template, respectively.");
    }

    // return nothing.
    return (None, None);
}







fn file_type(args: &Vec<Token>) {
    let command = SubCommand::Type;
    let result = check_args(args, command.clone());

    if result == (None, None) {
        // return with nothing. SubCommand terminated.
        return;
    }

    let paths = fs::read_dir(result.0.as_ref().unwrap()).unwrap();
    let paths_parent = result.0.as_ref().unwrap().display().to_string(); // As a String
    let parent = result.0.unwrap(); // PathBuf
    println!("CURRENT PATH: {}", &paths_parent);

    let mut files: Vec<DirEntry> = vec![];
    let mut file_types: Vec<String> = vec![];

    for path in paths {
        // unwrap path to get Ok(path) i.e. DirEntry
        let path = path.unwrap();
        //println!("FILE: {:?}", &path.path());
        // get metadata for path then unwrap to get Ok() value instead of Err()
        let md = path.metadata().unwrap();
        if md.is_file() {
            let filename = &path.file_name();
            files.push(path);

            // Will panic when it encounters file with no extension.
            //let f_type = Path::new(filename).extension().and_then(OsStr::to_str).unwrap().to_string();
            
            // One method
            //let f_type = Path::new(filename).extension().and_then(OsStr::to_str);
            //let ff: String;
            //if let Some(f) = f_type {
            //    ff = f.to_string();
            //    if !file_types.contains(&ff) {
            //        file_types.push(ff);
            //    }
            //}
            //else {
            //    println!("PANICKED!!!");
            //    return;
            //}

            
            //if !file_types.contains(&ff) {
            //    file_types.push(ff);
            //}

            let f_type = Path::new(filename).extension().and_then(OsStr::to_str);
            let ff: String;
            match f_type {
                Some(f) => {
                    ff = f.to_string();
                    if !file_types.contains(&ff) {
                        file_types.push(ff);
                    }
                },
                None => {}
            };
        }
        else {
            // Ignore directories (for now)
            continue;
        }
    }

    // Not that this will not run if there are files with no file extensions
    if *&files.len() == 0 { // dereference, otherwise &usize will be compared to int
        println!("There are no files to sort");
        return;
    }
    println!("Found {} files with {} unique file types", &files.len(), &file_types.len());

    // Create file type paths 
    for file_type in &file_types {
        let mut full_path = parent.clone(); 
        // cannot reference (&) since we would be pushing to that reference below:
        full_path.push(&file_type);

        //println!("Full path: {:?}", &full_path);
        if !Path::new(&full_path).exists() {
            let f = fs::create_dir(&full_path);
            match f {
                Ok(_) => {
                    println!("New folder for '.{}' with name '{}' has been created\n-->  Full path: \"{}\"", 
                    &file_type, &file_type, &full_path.display());
                },
                Err(error) => {
                    println!("There was a problem creating the folder for \"{}\":\n{:?}", &file_type, error)
                }
            };
        }
    }

    let mut files_sorted:f64 = 0.0;
    let start = Instant::now();
    // TODO: maybe have progress bar
    let mut stdout = stdout();
    for (idx, file) in files.iter().enumerate() {
        let done = idx as f64 / *&files.len() as f64;
        // get file extension
        //let ext = Path::new(&file.file_name()).extension().and_then(OsStr::to_str).unwrap().to_string();
        let fname = &file.file_name();
        // If Path::new(&file.file_name()) is used, rustc(E0716) is raised. 
        // It talks about how value is dropped when ext is matched, perhaps &file dropped? 
        // Bug?
        let ext = Path::new(fname).extension().and_then(OsStr::to_str);
        let ff: String;
        match ext {
            Some(f) => {
                ff = f.to_string();
            },
            None => {continue;}
        };

        
        // get original directory and navigate to file type directories
        let full_path = parent.clone().join(ff);

        //println!("FROM: {:?} --TO: {:?}", &file.path(), &full_path.join(file.file_name()));
        let f = fs::rename(file.path(), full_path.join(file.file_name()));
        match f {
            Ok(_) => {
                files_sorted += 1.0;
            },
            Err(error) => println!("There was a problem opening the file:\n{:?}", error)
        };
        
        print!("\rProcessing {:.1}%", done * 100.0);
        stdout.flush().unwrap();
        //sleep(Duration::from_millis(10));

    }
    let duration = start.elapsed();
    stdout.flush().unwrap();
    // \t doesn't seem to actually work in clearing everything - you get "Processed 100%9%"
    print!("\rProcessed 100%   \n"); 
    println!("Time taken: {:?}", duration);
    println!("Sorted {}/{} files into folders", &files_sorted, &files.len());

    //println!("{:?}", &files);
    //println!("{:?}", &file_types);

}

fn file_name(args: &Vec<Token>) {
    let command = SubCommand::Name;
    let result = check_args(args, command.clone());

    if result == (None, None) {
        // return with nothing. SubCommand terminated.
        return;
    }

}

fn file_date_mod(args: &Vec<Token>) {
    let command = SubCommand::Name;
    let result = check_args(args, command.clone());

    if result == (None, None) {
        // return with nothing. SubCommand terminated.
        return;
    }

}


// TODO: Keep improving command line parser
// TODO: address use of process::exit()
// TODO: Add option for config file to automate things more
// TODO: Add option to sort folders as well as files
fn main() {
    // TODO: Add option to break up an option like "-rf" and separate into "-r" and "-f" for parsing purposes
    let arg_list = parse_args();
    //let argc = arg_list.len();
    parser::tokenizer();

    if DEBUG {
        println!("{:?}", arg_list);
        for (pos, arg) in arg_list.iter().enumerate() {
            println!("Pos: {};  Arg: \"{}\"", pos, arg.value);
        }
    }

    // arg_list will contain at least one argument.
    let command = &arg_list[0].value;
    if ["-h", "--help", "help"].contains(&&command[..]) {
        help::help();
        // TODO: <command> -h will be handled by the command itself
    }
    else if command == &String::from("type") {
        file_type(&arg_list);
    }
    else if command == &String::from("name") {
        file_name(&arg_list);
    }
    else if command == &String::from("date") {
        file_date_mod(&arg_list);
    }
    else {
        println!("The command \"{}\" does not exist.", command);
    }


    // END
    //

}
