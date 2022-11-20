use std::fs::{self, DirEntry};
use std::io::{stdout, Write};
use std::path::{PathBuf, Path};
use std::time::Instant;

use clap::{Arg, ArgMatches, Command};

use crate::path::get_path;

pub fn cli() -> Command {
    Command::new("size")
        .about("Sort files by their size in KB (do not include 'KB' in the actual command)")
        .alias("S")
        .args([
            Arg::new("template")
                .short('t')
                .long("template")
                .help("The path you are using is a predefined one (e.g. 'downloads' for your downloads folder)")
                .action(clap::ArgAction::SetTrue),
            Arg::new("min")
                .long("min")
                .value_name("size")
                .help("Get files that are GREATER THAN the specified size (in KB)")
                .action(clap::ArgAction::Set),
            Arg::new("max")
                .long("max")
                .value_name("size")
                .help("Get files that are LESS THAN the specified size (in KB)")
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
    let mut size_min: Option<String> = None;
    let mut size_max: Option<String> = None;

    let use_template = args.get_flag("template");

    if let Some(p) = args.get_one::<String>("path") {
        path = get_path(p, use_template);
    }
    if let Some(min) = args.get_one::<String>("min") {
        size_min = Some(min.to_string());
    }
    if let Some(max) = args.get_one::<String>("max") {
        size_max = Some(max.to_string());
    }


    if path == None {
        println!("ERROR: The path is invalid");
        return;
    }

    // Neither was provided
    if size_min == None && size_max == None {
        println!("ERROR: A min or max size (or both) must be provided");
        return;
    }

    let mut has_min = false;
    let mut has_max = false;
    let mut min: u64 = 0;
    let mut max: u64 = 0;

    if size_min != None {
        has_min = true;
        min = size_min.clone().unwrap().parse::<u64>().unwrap();
        if &min < &0 {
            println!("ERROR: Negative values cannot be used as file sizes");
        return;
        }
    }
    if size_max != None {
        has_max = true;
        max = size_max.clone().unwrap().parse::<u64>().unwrap();
        if &max < &0 {
            println!("ERROR: Negative values cannot be used as file sizes");
        return;
        }
    }

    let dir = fs::read_dir(path.as_ref().unwrap()).unwrap();
    let paths_parent = path.as_ref().unwrap().display().to_string(); // As a String
    let parent = path.unwrap(); // PathBuf
    println!("CURRENT PATH: {}", &paths_parent);

    let mut files: Vec<DirEntry> = vec![];

    for item in dir {
        let item = item.unwrap();
        let md = item.metadata().unwrap();

        if md.is_file() {
            let file_size = &md.len() / 1000; // Convert bytes to kilobytes

            //println!("filesize:{} - min-check:{:?}, max-check:{:?}", &file_size, file_size >= min, file_size <= max);
            if (has_min && has_max) && (file_size >= min && file_size <= max) {
                files.push(item);
            }
            else if (has_min && !has_max) && file_size >= min {
                files.push(item);
            }
            else if (has_max && !has_min) && file_size <= max {
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
    let size = "Sorted_By_Size".to_string();
    full_path.push(&size);
    if !Path::new(&full_path).exists() {
        let f = fs::create_dir(&full_path);
        match f {
            Ok(_) => {
                println!("New folder '{}' has been created\n-->  Full path: \"{}\"", 
                    &size, &full_path.display());
            }
            Err(error) => {
                println!("There was a problem creating the folder for \"{}\":\n{:?}", &size, error)
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
