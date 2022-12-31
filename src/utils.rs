use std::fs::{self, DirEntry};
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use chrono::{NaiveDateTime, Utc};

pub fn get_current_path(path: Option<PathBuf>) -> PathBuf {
    let paths_parent = path.as_ref().unwrap().display().to_string();
    println!("CURRENT PATH: {}", &paths_parent);
    path.unwrap()
}

pub fn set_folder_name(suffix: String) -> String {
    let timestamp = Utc::now().timestamp();
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let n = naive.format("%Y%m%dT%H%M%S");
    format!("Cabinet-{n}-{suffix}")
}

/// Create the folder to store the sorted files in
pub fn create_folder(path: &PathBuf, name: &String) {
    if !Path::new(&path).exists() {
        let f = fs::create_dir(&path);
        match f {
            Ok(_) => {
                println!(
                    "New folder '{}' has been created\n --> \"{}\"",
                    &name,
                    &path.display()
                );
            }
            Err(error) => {
                println!(
                    "There was a problem creating the folder for \"{}\":\n{:?}",
                    &name, error
                );
            }
        }
    }
}

// TODO: Consider adding a progress bar
pub fn sort_files(path: &PathBuf, files: &Vec<DirEntry>) {
    let mut files_sorted: f64 = 0.0;
    let start = Instant::now();
    let mut stdout = stdout();

    for (idx, file) in files.iter().enumerate() {
        let done = idx as f64 / *&files.len() as f64;
        let f = fs::rename(file.path(), path.join(file.file_name()));
        match f {
            Ok(_) => files_sorted += 1.0,
            Err(error) => println!("There was a problem opening the file:\n{:?}", error),
        }

        print!("\rProcessing {:.1}%", done * 100.0);
        stdout.flush().unwrap();
    }

    let duration = start.elapsed();
    stdout.flush().unwrap();

    // NOTE: \t doesn't seem to actually work in clearing everything - you get "Processed 100%9%"
    print!("\rProcessed 100%   \n");
    println!("Time taken: {:?}", duration);
    println!(
        "Sorted {}/{} files into folders",
        &files_sorted,
        &files.len()
    );
}
