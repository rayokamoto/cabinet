pub mod help;
pub mod file_type;
pub mod file_name;
pub mod file_date;

pub fn get_subcommands() -> Vec<String> {
    vec![
        String::from("type"),
        String::from("name"),
        String::from("date"),
    ]
}
