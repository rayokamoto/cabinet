
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

pub fn cmd_help(command: SubCommand) {

    if command == SubCommand::Type {
        print!("command: type
Usage: cab type [<options>] <path>
    -p, --path      The path you are using is an absolute path
    -t, --template  The path you are using is a predefined one. E.g. downloads for your downloads folder
")
    }
    else if command == SubCommand::Name {
        print!("command: name
Usage: cab name [<options>] <path>
    
")
    }
    else if command == SubCommand::Date {
        print!("command: date
Usage: cab date [<options>] <path>

")
    }
    else {
        println!("The command either does not exist or no documentation currently exists for it.");
    }

}
