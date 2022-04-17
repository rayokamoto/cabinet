//use crate::parser;
//use crate::parser::SubCommand;

pub fn help() {
    print!("Cabinet 0.1.0
Usage: cab <command> [<options>] <path>

Available commands:
   help      Shows this command
   type      Sort files by file type
   name      Sort files by file name
   date      Sort files by their date of modification

Use -h or --help for more information on a command. 
")
}

// TODO: SubCommand::<subcommand> or use of string??

pub fn cmd_help<S: AsRef<str>>(command: S) {
    let command = command.as_ref();


    if command == "type" {
        print!("command: type
Usage: cab type [<options>] <path>
    -a, --absolute  The path you are using is an absolute path
    -t, --template  The path you are using is a predefined one. E.g. downloads for your downloads folder
")
    }
    else if command == "name" {
        print!("command: name
Usage: cab name [<options>] <path>
    -a, --absolute      The path you are using is an absolute path
    -t, --template      The path you are using is a predefined one. E.g. downloads for your downloads folder
    --includes <match>  File name includes...
    --excludes <match>  File name excludes...
\nWARNING: CURRENTLY NOT IMPLEMENTED!
")
    }
    else if command == "date" {
        print!("command: date
Usage: cab date [<options>] <path>
    -a, --absolute  The path you are using is an absolute path
    -t, --template  The path you are using is a predefined one. E.g. downloads for your downloads folder
    --before <date> Get files from before specified date. Date format is YYYY-MM-DD
    --after <date>  Get files from after specified date. Date format is YYYY-MM-DD
\nWARNING: CURRENTLY NOT IMPLEMENTED!
")
    }
    // This should never be called
    else {
        println!("The command either does not exist or no documentation currently exists for it.");
    }

}
