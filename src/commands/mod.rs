use clap::Command;

pub mod date;
pub mod file_type;
pub mod multisort;
pub mod name;
pub mod size;

pub fn builtin() -> Vec<Command> {
    vec![
        date::cli(),
        multisort::cli(),
        name::cli(),
        size::cli(),
        file_type::cli(),
    ]
}
