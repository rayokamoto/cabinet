use std::fs::{self, DirEntry};
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::{Instant, UNIX_EPOCH};

use chrono;
use regex::Regex;

use crate::parser::Token;
use crate::path::{get_path, ArgType};

fn next_arg(argc: &usize, argv: &Vec<Token>) -> Token {
    assert!(argc > &0);
    //argc - 1;
    let arg_len = &argv.len();
    argv[arg_len - argc].clone()
}

pub fn file_date(args: &Vec<Token>) {
    let mut argc = args.len() - 1; // since we want to ignore subcommand itself
    
    let mut path_type = ArgType::Absolute;
    let mut path: Option<PathBuf> = None;
    let mut date: Option<String> = None;
    let mut date_expected = false;

    // TODO: Make it so that you can set a date range by invoking both --after and --before
    // For checking if one or both dates were specified by user
    let mut date_after = false;
    let mut date_before = false;

    // Date pattern for YYYY-MM-DD
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    while argc > 0 {
        let arg = next_arg(&argc, &args);
        argc = argc - 1;
        
        if date_expected {
            if re.is_match(&arg.value) {
                date = Some(arg.value.clone());
                date_expected = false;
            }
            else {
                println!("The provided date format was invalid. Please provide the date in YYYY-MM-DD format");
                exit(1);
            }
            continue;
        }

        if ["--before"].contains(&&arg.value[..]) {
            date_expected = true;
            date_before = true;
        }
        else if ["--after"].contains(&&arg.value[..]) {
            date_expected = true;
            date_after = true;
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

    // add logic for if path exists or not


    //println!("Path type: {:?}", &path_type);
    //println!("Date: {:?}", &date);
    
    if path == None {
        // no path or invalid path
        return;
    }


    // TODO: Implement this feature for directories/folders

    let dir = fs::read_dir(path.as_ref().unwrap()).unwrap();
    let paths_parent = path.as_ref().unwrap().display().to_string(); // As a String
    let parent = path.unwrap(); // PathBuf
    println!("CURRENT PATH: {}", &paths_parent);

    let mut files: Vec<DirEntry> = vec![];

    for item in dir {
        let item = item.unwrap();
        let md = item.metadata().unwrap();
        //let md = fs::metadata(item)?; // Alternative

        if md.is_file() {
            //if let Ok(time) = md.modified() {
            //    println!("{:?}", time);
            //} else {
            //    println!("Not supported on this platform");
            //}

            let time = md.modified().unwrap().duration_since(UNIX_EPOCH).unwrap(); // TODO: error handling
            let dur = chrono::Duration::from_std(time).unwrap();
            
            // this is needed otherwise parsing won't work. maybe because re was consumed earlier?
            let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
            //let text = "2012-03-14, 2013-01-01 and 2014-07-05";
            //for cap in re.captures_iter(text) {
            //    println!("Day: {} Month: {} Year: {}", &cap[3], &cap[2], &cap[1]);
            //}
            
            let text = &date.as_ref().unwrap()[..];
            let cap = re.captures(text).unwrap();
            //println!("Day: {} Month: {} Year: {}", &cap[3], &cap[2], &cap[1]);
            
            let date_time = chrono::NaiveDate::from_ymd(cap[1].parse::<i32>().unwrap(), cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap()).and_hms(0, 0, 0);
            
            let user_date = date_time.timestamp();
            let file_date = dur.num_seconds();
            
            if date_before && file_date <= user_date {
                files.push(item);
            }
            else if date_after && file_date >= user_date {
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
    let date = date.unwrap();
    let mut full_path = parent.clone();
    full_path.push(&date);
    if !Path::new(&full_path).exists() {
        let f = fs::create_dir(&full_path);
        match f {
            Ok(_) => {
                println!("New folder '{}' has been created\n-->  Full path: \"{}\"", 
                    &date, &full_path.display());
            }
            Err(error) => {
                println!("There was a problem creating the folder for \"{}\":\n{:?}", &date, error)
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
