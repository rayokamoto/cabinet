
use std::path::PathBuf;

use regex::Regex;

/// Sort by date
pub fn date(path: Option<PathBuf>, use_template: bool) {
    // Date pattern for YYYY-MM-DD
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    if path == None {
        println!("ERROR: The path is invalid");
        return;
    }
}
