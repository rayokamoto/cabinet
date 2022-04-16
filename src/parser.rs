use std::env::args_os;
use std::ffi::OsString;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    SubCommand,
    Argument,
    ArgumentValue
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) of_type: TokenType,
    pub(crate) value: String,
    pub(crate) position: usize
}


#[derive(Debug, PartialEq, Clone)]
pub enum SubCommand {
    //Help,
    Type,
    Name,
    Date
}


pub fn tokenizer() -> Vec<Token> {
    let mut arg_list: Vec<Token> = Vec::new();

    let mut argv: Vec<OsString> = args_os().collect();
    argv.remove(0); // Remove the filename from the array
    let argc = argv.len();

    if argc == 0 {
        return arg_list;
    }

    for (idx, arg) in argv.iter().enumerate() {
        //
    }


    arg_list    
}

