//use crate::parser;
//use crate::parser::SubCommand;

/// The primary help command. Shows overall usage info
pub fn help() {
    print!("Cabinet 0.2.1
Usage: cab <command> [<options>] <path>

Available commands:
   help      Shows this command
   type      Sort files by file type
   date      Sort files by their date of modification
   name      Sort files by file name
   size      Sort by file size (in KB)

Use -h or --help for more information on a command. 
")
}

// TODO: SubCommand::<subcommand> or use of string??

/// Subcommand-specific help
pub fn cmd_help<S: AsRef<str>>(command: S) {
    let command = command.as_ref();


    if command == "type" {
        print!("command: type
Sort files by file type

Usage: cab type [<options>] <path>
    -p, --path      The path you are using is an absolute or relative path. This is the default option
    -t, --template  The path you are using is a predefined one. E.g. downloads for your downloads folder
")
    }
    else if command == "name" {
        print!("command: name
Sort files by their name

Usage: cab name [<options>] <path>
    -p, --path          The path you are using is an absolute or relative path. This is the default option
    -t, --template      The path you are using is a predefined one. E.g. downloads for your downloads folder
    --includes <match>  File name includes...
    --excludes <match>  File name excludes...
")
    }
    else if command == "date" {
        print!("command: date
Sort files by the date modified

Usage: cab date [<options>] <path>
    -p, --path      The path you are using is an absolute or relative path. This is the default option
    -t, --template  The path you are using is a predefined one. E.g. downloads for your downloads folder
    --before <date> Get files from before specified date. Date format is YYYY-MM-DD
    --after <date>  Get files from after specified date. Date format is YYYY-MM-DD
")
    }
    else if command == "size" {
        print!("command: size
Sort files by their size in KB (do not include 'KB' in the actual command)

Usage: cab size [<options>] <path>
    -p, --path      The path you are using is an absolute or relative path. This is the default option
    -t, --template  The path you are using is a predefined one. E.g. downloads for your downloads folder
    --lt <size>     Get files that are LESS THAN the specified size (in KB)
    --gt <size>     Get files that are GREATER THAN the specified size (in KB)
")
    }
    // This should never be called
    else {
        println!("The command either does not exist or no documentation currently exists for it.");
    }

}
