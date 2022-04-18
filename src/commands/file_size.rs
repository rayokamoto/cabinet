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

pub fn file_size(args: &Vec<Token>) {
    let mut argc = args.len() - 1; // since we want to ignore subcommand itself

    let mut path: Option<PathBuf> = None;
    let mut path_type = ArgType::Absolute;
    let mut size: Option<String> = None;
    let mut size_expected = false; // expecting size as next argument

    // TODO: Enable use of both a less-than and greater-than pattern
    let mut size_lt = false;
    let mut size_gt = false;

    while argc > 0 {
        let arg = next_arg(&argc, &args);
        argc = argc - 1;
        
        if size_expected {
            if size != None {
                println!("You have already provided a file size!");
                exit(1);
            }
            size = Some(arg.value.clone());
            size_expected = false;
            continue;
        }

        if ["--lt"].contains(&&arg.value[..]) {
            size_expected = true;
            size_lt = true;
        }
        else if ["--gt"].contains(&&arg.value[..]) {
            size_expected = true;
            size_gt = true;
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
        // no path or invalid path
        println!("ERROR: There was no path provided, or the path was invalid");
        return;
    }

    // Neither was provided
    if (!size_lt && !size_gt) || size == None {
        println!("ERROR: A before or after date must be provided");
        return;
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
            let size = size.clone().unwrap().parse::<u64>().unwrap();
            let file_size = &md.len() / 1000; // Convert bytes to kilobytes
            
            if size_lt && file_size <= size {
                files.push(item);
            }
            else if size_gt && file_size >= size {
                files.push(item);
            }

        }

        // continue for now
    }

    if *&files.len() == 0 {
        println!("There are no files to sort that match the given parameters");
        return;
    }
    println!("Found {} files that are able to be sorted", &files.len());

    // Make folder if necessary
    // TODO: Sort out the naming of the newly created folder
    let mut full_path = parent.clone();
    let size = size.unwrap();
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
