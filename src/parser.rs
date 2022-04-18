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


//#[derive(Debug, PartialEq, Clone)]
//pub enum SubCommand {
//    //Help,
//    Type,
//    Name,
//    Date
//}
