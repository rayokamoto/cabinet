use std::path::{PathBuf, Path};
use std::fs::{self, DirEntry};
use std::ffi::OsStr; 
use std::time::Instant;
use std::io::{stdout, Write};

use clap::{Arg, ArgMatches, Command};

use crate::path::get_path;

pub fn cli() -> Command {
    Command::new("name")
        .about("Sort files by file name")
        .alias("N")
        .args([
            Arg::new("template")
                .short('t')
                .long("template")
                .help("The path you are using is a predefined one (e.g. 'downloads' for your downloads folder)")
                .action(clap::ArgAction::SetTrue),
            Arg::new("includes")
                .long("includes")
                .value_name("match")
                .help("File name includes...")
                .action(clap::ArgAction::Set),
            Arg::new("excludes")
                .long("excludes")
                .value_name("match")
                .help("File name excludes...")
                .action(clap::ArgAction::Set),
        ])
        .arg_required_else_help(true)
        .arg(
            Arg::new("path")
            .action(clap::ArgAction::Set)
            .value_name("PATH")
            .required(true)
        )
        .subcommand_value_name("PATH")
}

pub fn exec(args: &ArgMatches) {
    let mut path: Option<PathBuf> = None;
    let mut include_pattern: Option<String> = None;
    let mut exclude_pattern: Option<String> = None;

    let use_template = args.get_flag("template");

    if let Some(p) = args.get_one::<String>("path") {
        path = get_path(p, use_template);
    }
    if let Some(incl) = args.get_one::<String>("includes") {
        include_pattern = Some(incl.to_string());
    }
    if let Some(excl) = args.get_one::<String>("excludes") {
        exclude_pattern = Some(excl.to_string());
    }

    if path == None {
        println!("ERROR: The path is invalid");
        return;
    }

    // Neither was provided
    if include_pattern == None && exclude_pattern == None {
        println!("ERROR: A before or after date must be provided");
        return;
    }

    let mut has_include = false;
    let mut has_exclude = false;
    let mut include = String::new();
    let mut exclude = String::new();

    if include_pattern != None {
        has_include = true;
        include = include_pattern.unwrap();
    }
    if exclude_pattern != None {
        has_exclude = true;
        exclude = exclude_pattern.unwrap();
    }

    let dir = fs::read_dir(path.as_ref().unwrap()).unwrap();
    let paths_parent = path.as_ref().unwrap().display().to_string(); // As a String
    let parent = path.unwrap(); // PathBuf
    println!("CURRENT PATH: {}", &paths_parent);

    let mut files: Vec<DirEntry> = vec![];

    for item in dir {
        let item = item.unwrap();
        let md = item.metadata().unwrap();

        let filename = &item.file_name(); // gets the name (no file extension)
        let f = OsStr::to_str(&filename).unwrap();

        if md.is_file() {
            if (has_include && has_exclude) && (f.contains(&include) && !f.contains(&exclude)) {
                files.push(item);
            }
            else if (has_include && !has_exclude) && f.contains(&include) {
                files.push(item);
            }
            // Only add file if it DOESN'T include the pattern (since it is exclude pattern)
            else if (has_exclude && !has_include) && !f.contains(&exclude) {
                files.push(item);
            }
        }
    }

    if *&files.len() == 0 {
        println!("There are no files to sort that match the given parameters");
        return;
    }
    println!("Found {} files that are able to be sorted", &files.len());

    // Make folder if necessary
    // TODO: Sort out the naming of the newly created folder
    let mut full_path = parent.clone();
    let folder = "Sorted_By_Name".to_string();
    full_path.push(&folder);
    if !Path::new(&full_path).exists() {
        let f = fs::create_dir(&full_path);
        match f {
            Ok(_) => {
                println!("New folder '{}' has been created\n-->  Full path: \"{}\"", 
                    &folder, &full_path.display());
            }
            Err(error) => {
                println!("There was a problem creating the folder for \"{}\":\n{:?}", &folder, error)
            }
        }
    }

    let mut files_sorted: f64 = 0.0;
    let start = Instant::now();
    let mut stdout = stdout();
    for (idx, file) in files.iter().enumerate() {
        let done = idx as f64 / *&files.len() as f64;
        //let full_path = parent.clone().join(&date);
        let f = fs::rename(file.path(), full_path.join(file.file_name()));
        match f {
            Ok(_) => files_sorted += 1.0,
            Err(error) => println!("There was a problem opening the file:\n{:?}", error)
        }

        print!("\rProcessing {:.1}%", done * 100.0);
        stdout.flush().unwrap();
    }
    let duration = start.elapsed();
    stdout.flush().unwrap();
    print!("\rProcessed 100%   \n"); 
    println!("Time taken: {:?}", duration);
    println!("Sorted {}/{} files into folders", &files_sorted, &files.len());
}
