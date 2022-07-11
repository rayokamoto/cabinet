use std::fs::{self, DirEntry};
use std::io::{stdout, Write};
use std::path::{PathBuf, Path};
use std::process::exit;
use std::time::Instant;

use crate::parser::Token;
use crate::path::{get_path, ArgType};

fn next_arg(argc: &usize, argv: &Vec<Token>) -> Token {
    assert!(argc > &0);
    //argc - 1;
    let arg_len = &argv.len();
    argv[arg_len - argc].clone()
}

/// Sort files by their size
pub fn file_size(args: &Vec<Token>) {
    let mut argc = args.len() - 1; // since we want to ignore subcommand itself

    let mut path: Option<PathBuf> = None;
    let mut path_type = ArgType::Absolute;

    // size_expected: 0 for not expecting (false), 1 for min size, 2 for max size
    let mut size_expected = 0;
    let mut size_min: Option<String> = None;
    let mut size_max: Option<String> = None;

    while argc > 0 {
        let arg = next_arg(&argc, &args);
        argc = argc - 1;
        
        // TODO: use match statement?
        if size_expected == 1 {
            if size_min != None {
                println!("You have already provided a minimum file size!");
                exit(1);
            }
            size_min = Some(arg.value.clone());
            size_expected = 0;
            continue;
        }
        else if size_expected == 2 {
            if size_max != None {
                println!("You have already provided a maximum file size!");
                exit(1);
            }
            size_max = Some(arg.value.clone());
            size_expected = 0;
            continue;
        }

        if ["--min"].contains(&&arg.value[..]) {
            size_expected = 1;
        }
        else if ["--max"].contains(&&arg.value[..]) {
            size_expected = 2;
        }
        else if ["-t", "--template"].contains(&&arg.value[..]) {
            if argc <= 0 {
                println!("No path provided!");
                exit(1);
            }
            path_type = ArgType::Template;
        }
        else if ["-p", "--path"].contains(&&arg.value[..]) {
            if argc <= 0 {
                println!("No path provided!");
                exit(1);
            }
            path_type = ArgType::Absolute;
        }
        else if arg.value.starts_with("--") || arg.value.starts_with("-"){
            println!("Not a valid argument/flag");
            exit(1);
        }
        else {
            // we assume that it is the path
            //println!("Assuming path was provided.");
            path = get_path(&arg.value, path_type);
            //break;
        }
    }


    if path == None {
        // No path or invalid path
        println!("ERROR: There was no path provided, or the path was invalid");
        return;
    }

    // Neither was provided
    if size_min == None && size_max == None {
        println!("ERROR: A min or max size (or both) must be provided");
        return;
    }

    if  size_min.clone().unwrap().parse::<i64>().unwrap() < 0 || size_max.clone().unwrap().parse::<i64>().unwrap() < 0 {
        println!("ERROR: Negative values cannot be used as file sizes");
        return;
    }

    let mut has_min = false;
    let mut has_max = false;
    let mut min: u64 = 0;
    let mut max: u64 = 0;

    if size_min != None {
        has_min = true;
        min = size_min.clone().unwrap().parse::<u64>().unwrap();
    }
    if size_max != None {
        has_max = true;
        max = size_max.clone().unwrap().parse::<u64>().unwrap();
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
