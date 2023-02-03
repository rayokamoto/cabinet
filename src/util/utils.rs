use std::fs::{self, DirEntry};
use std::io::{self, stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::rc::Rc;
use std::time::Instant;

use chrono::{NaiveDateTime, Utc};

// TODO: Use local time instead of UTC

/// Set folder name according to the following format of `Cabinet-YYYYmmddTHHMMSS-<suffix>`
pub fn set_folder_name(suffix: String) -> String {
    let timestamp = Utc::now().timestamp();
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let n = naive.format("%Y%m%dT%H%M%S");
    format!("Cabinet-{n}-{suffix}")
}

/// Create the folder to store the sorted files in. If folder name already exists,
/// ask user if they wish to proceed or not.
pub fn create_folder(path: PathBuf, folder: String) -> Result<PathBuf, io::Error> {
    let mut path = path;
    path.push(&folder);

    if Path::new(&path).exists() {
        print!("The folder with the name '{folder}' already exists. Sorted files will be placed in this folder anyway. Proceed? [y/N] ");
        let _ = stdout().flush();
        let mut ans = String::new();
        stdin().read_line(&mut ans).expect("Malformed input");
        if let Some('\n') = ans.chars().next_back() {
            ans.pop();
        }
        if let Some('\r') = ans.chars().next_back() {
            ans.pop();
        }

        if ans == "y" {
            println!("\nContinuing anyway...");
            return Ok(path);
        } else {
            println!("\nAborted.");
            exit(0);
        }
    }

    let f = fs::create_dir(&path);
    match f {
        Ok(_) => {
            println!(
                "New folder '{}' has been created\n --> \"{}\"",
                &folder,
                &path.display()
            );
            return Ok(path);
        }
        Err(error) => {
            println!(
                "There was a problem creating the folder for \"{}\":\n{:?}",
                &folder, error
            );
            return Err(error);
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

pub fn sort_files_rc(path: &PathBuf, files: &Vec<Rc<DirEntry>>) {
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
