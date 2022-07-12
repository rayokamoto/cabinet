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

    // 0 - no date, 1 - date before, 2 - date after
    let mut date_expected = 0;
    let mut date_after: Option<String> = None;
    let mut date_before: Option<String> = None;

    // Date pattern for YYYY-MM-DD
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    while argc > 0 {
        let arg = next_arg(&argc, &args);
        argc = argc - 1;


        if date_expected == 1 {
            if date_before != None {
                println!("You have already provided a before date!");
                exit(1);
            }
            if re.is_match(&arg.value) {
                date_before = Some(arg.value.clone());
                date_expected = 0;
            }
            else {
                println!("The provided date format was invalid. Please provide the date in YYYY-MM-DD format");
                exit(1);
            }
            continue;
        }
        else if date_expected == 2 {
            if date_after != None {
                println!("You have already provided an after date!");
                exit(1);
            }
            if re.is_match(&arg.value) {
                date_after = Some(arg.value.clone());
                date_expected = 0;
            }
            else {
                println!("The provided date format was invalid. Please provide the date in YYYY-MM-DD format");
                exit(1);
            }
            continue;
        }


        if ["--before"].contains(&&arg.value[..]) {
            date_expected = 1;
        }
        else if ["--after"].contains(&&arg.value[..]) {
            date_expected = 2;
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


    //println!("Path type: {:?}", &path_type);
    //println!("Date: {:?}", &date);

    if path == None {
        // No path or invalid path
        println!("ERROR: There was no path provided, or the path was invalid");
        return;
    }

    // Neither was provided
    if date_before == None && date_after == None {
        println!("ERROR: A before or after date must be provided");
        return;
    }

    // TODO: Implement this feature for directories/folders
    // TODO: Deal with symlinks

    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

    let mut has_before = false;
    let mut has_after = false;
    let mut before: i64 = 0;
    let mut after: i64 = 0;

    if date_before != None {
        has_before = true;
        let d = &date_before.as_ref().unwrap()[..];
        let cap = re.captures(d).unwrap();
        //let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        //for cap in re.captures_iter(text) {
        //    println!("Day: {} Month: {} Year: {}", &cap[3], &cap[2], &cap[1]);
        //}

        let date_time = chrono::NaiveDate::from_ymd(cap[1].parse::<i32>().unwrap(), cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap()).and_hms(0, 0, 0);
        before = date_time.timestamp();
    }
    if date_after != None {
        has_after = true;
        let d = &date_after.as_ref().unwrap()[..];
        let cap = re.captures(d).unwrap();
        let date_time = chrono::NaiveDate::from_ymd(cap[1].parse::<i32>().unwrap(), cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap()).and_hms(0, 0, 0);
        after = date_time.timestamp();
    }


    let dir = fs::read_dir(path.as_ref().unwrap()).unwrap();
    let paths_parent = path.as_ref().unwrap().display().to_string(); // As a String
    let parent = path.unwrap(); // PathBuf
    println!("CURRENT PATH: {}", &paths_parent);

    let mut files: Vec<DirEntry> = vec![];

    for item in dir {
        let item = item.unwrap();
        let md = item.metadata().unwrap();
        //let md = fs::metadata(item)?; // Alternative method

        if md.is_file() {
            //if let Ok(time) = md.modified() {
            //    println!("{:?}", time);
            //} else {
            //    println!("Not supported on this platform");
            //}

            let time = md.modified().unwrap().duration_since(UNIX_EPOCH).unwrap(); // TODO: error handling
            let dur = chrono::Duration::from_std(time).unwrap();
            let file_date = dur.num_seconds();

            if (has_before && has_after) && (file_date >= after && file_date <= before) {
                files.push(item);
            }
            else if (has_before && !has_after) && file_date <= before {
                files.push(item);
            }
            else if (has_after && !has_before) && file_date >= after {
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
    let date = "Sorted_By_Date".to_string();
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
