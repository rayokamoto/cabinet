use std::fs::{self, DirEntry};
use std::io::{stdout, Write};
use std::path::{PathBuf, Path};
use std::time::{UNIX_EPOCH, Instant};

use chrono::{self, NaiveDateTime};
use clap;
use clap::{Arg, ArgMatches, Command};
use regex::Regex;

use crate::path::get_path;

pub fn cli() -> Command {
    Command::new("date")
        .about("Sort files by their date of modification")
        .alias("D")
        .args([
            Arg::new("template")
                .short('t')
                .long("template")
                .help("The path you are using is a predefined one (e.g. 'downloads' for your downloads folder)")
                .action(clap::ArgAction::SetTrue),
            Arg::new("before")
                .long("before")
                .value_name("date")
                .help("Get files from before the specified date. Date format is YYYY-MM-DD")
                .action(clap::ArgAction::Set),
            Arg::new("after")
                .long("after")
                .value_name("date")
                .help("Get files from after the specified date. Date format is YYYY-MM-DD")
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
    let mut date_after: Option<String> = None;
    let mut date_before: Option<String> = None;

    // Date pattern for YYYY-MM-DD
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    let use_template = args.get_flag("template");

    if let Some(p) = args.get_one::<String>("path") {
        path = get_path(p, use_template);
    }
    if let Some(after) = args.get_one::<String>("after") {
        if re.is_match(after) {
            date_after = Some(after.to_string());
        }
    }
    if let Some(before) = args.get_one::<String>("before") {
        if re.is_match(before) {
            date_before = Some(before.to_string());
        }
    }

    if path == None {
        println!("ERROR: The path is invalid");
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

        let date_time = chrono::NaiveDate::from_ymd_opt(cap[1].parse::<i32>().unwrap(), cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap());
        let naive_date_time: NaiveDateTime;
        match date_time {
            Some(d) => naive_date_time = d.and_hms_opt(0, 0, 0).unwrap(),
            None => {
                println!("ERROR: Invalid date conversion");
                return;
            }
        }
        before = naive_date_time.timestamp();
    }
    if date_after != None {
        has_after = true;
        let d = &date_after.as_ref().unwrap()[..];
        let cap = re.captures(d).unwrap();
        let date_time = chrono::NaiveDate::from_ymd_opt(cap[1].parse::<i32>().unwrap(), cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap());
        let naive_date_time: NaiveDateTime;
        match date_time {
            Some(d) => naive_date_time = d.and_hms_opt(0, 0, 0).unwrap(),
            None => {
                println!("ERROR: Invalid date conversion");
                return;
            }
        }

        after = naive_date_time.timestamp();
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
