pub mod help;
pub mod file_date;
pub mod file_name;
pub mod file_size;
pub mod file_type;

pub fn get_subcommands() -> Vec<String> {
    vec![
        String::from("date"),
        String::from("name"),
        String::from("size"),
        String::from("type"),
    ]
}
