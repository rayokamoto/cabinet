use std::ffi::OsStr;
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

pub fn file_name(args: &Vec<Token>) {
    let mut argc = args.len() - 1; // since we want to ignore subcommand itself

    let mut path: Option<PathBuf> = None;
    let mut path_type = ArgType::Absolute;
    let mut name_pattern: Option<String> = None;
    let mut name_pattern_expected = false;

    // TODO: Enable use of both an include and exclude pattern
    let mut name_include_pattern = false;
    let mut name_exclude_pattern = false;


    while argc > 0 {
        let arg = next_arg(&argc, &args);
        argc = argc - 1;
        

        if name_pattern_expected {
            if name_pattern != None {
                println!("You have already provided an include/exclude pattern!");
                exit(1);
            }
            name_pattern = Some(arg.value.clone());
            name_pattern_expected = false;
            continue;
        }


        if ["--includes"].contains(&&arg.value[..]) {
            name_pattern_expected = true;
            name_include_pattern = true;
        }
        else if ["--excludes"].contains(&&arg.value[..]) {
            name_pattern_expected = true;
            name_exclude_pattern = true;
        }
        else if ["-t", "--template"].contains(&&arg.value[..]) {
            if argc <= 0 {
                println!("No path provided!");
                exit(1);
            }
            path_type = ArgType::Template;
        }
        else if ["-a", "--absolute"].contains(&&arg.value[..]) {
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
        // no path or invalid path
        println!("ERROR: There was no path provided, or the path was invalid");
        return;
    }

    // Neither was provided
    if (!name_include_pattern && !name_exclude_pattern) || name_pattern == None {
        println!("ERROR: A before or after date must be provided");
        return;
    }

    let dir = fs::read_dir(path.as_ref().unwrap()).unwrap();
    let paths_parent = path.as_ref().unwrap().display().to_string(); // As a String
    let parent = path.unwrap(); // PathBuf
    println!("CURRENT PATH: {}", &paths_parent);

    let mut files: Vec<DirEntry> = vec![];

    let name_pattern = name_pattern.unwrap();

    for item in dir {
        let item = item.unwrap();
        let md = item.metadata().unwrap();

        let filename = &item.file_name(); // gets the name (no file extension)
        let filename = OsStr::to_str(&filename).unwrap();

        if md.is_file() {
            if name_include_pattern && filename.contains(&name_pattern) {
                files.push(item);
            }
            // Only add file if it DOESN'T include the pattern (since it is exclude pattern)
            else if name_exclude_pattern && !filename.contains(&name_pattern){
                files.push(item);
            }
        }
        else {
            continue;
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
    full_path.push(&name_pattern);
    if !Path::new(&full_path).exists() {
        let f = fs::create_dir(&full_path);
        match f {
            Ok(_) => {
                println!("New folder '{}' has been created\n-->  Full path: \"{}\"", 
                    &name_pattern, &full_path.display());
            }
            Err(error) => {
                println!("There was a problem creating the folder for \"{}\":\n{:?}", &name_pattern, error)
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
