use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use chrono::{self, NaiveDateTime};
use clap;
use clap::{Arg, ArgMatches, Command};
use regex::Regex;

use crate::util;
use crate::util::path::{get_current_path, get_path};

pub fn cli() -> Command {
    Command::new("date")
        .about("Sort files by their date of modification")
        .args([
            Arg::new("before")
                .short('B')
                .long("before")
                .value_name("date")
                .help("Get files from before the specified date. Date format is YYYY-MM-DD")
                .action(clap::ArgAction::Set),
            Arg::new("after")
                .short('A')
                .long("after")
                .value_name("date")
                .help("Get files from after the specified date. Date format is YYYY-MM-DD")
                .action(clap::ArgAction::Set),
            Arg::new("template")
                .short('t')
                .long("template")
                .help("The path you are using is a predefined one (e.g. 'downloads' for your downloads folder)")
                .action(clap::ArgAction::SetTrue),
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
    let use_template = args.get_flag("template");

    if let Some(p) = args.get_one::<String>("path") {
        path = get_path(p, use_template);
    }
    if path == None {
        println!("ERROR: The path is invalid");
        return;
    }

    let mut date_after: Option<String> = None;
    let mut date_before: Option<String> = None;

    // Date pattern for YYYY-MM-DD
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

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

    if date_before == None && date_after == None {
        println!("ERROR: A before or after date must be provided");
        return;
    }

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

        let date_time = chrono::NaiveDate::from_ymd_opt(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
        );
        let naive_date_time: NaiveDateTime;
        match date_time {
            Some(d) => naive_date_time = d.and_hms_opt(0, 0, 0).unwrap(),
            None => {
                println!("ERROR: Invalid date conversion");
                return;
            }
        }
        before = naive_date_time.and_utc().timestamp();
    }
    if date_after != None {
        has_after = true;
        let d = &date_after.as_ref().unwrap()[..];
        let cap = re.captures(d).unwrap();
        let date_time = chrono::NaiveDate::from_ymd_opt(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
        );
        let naive_date_time: NaiveDateTime;
        match date_time {
            Some(d) => naive_date_time = d.and_hms_opt(0, 0, 0).unwrap(),
            None => {
                println!("ERROR: Invalid date conversion");
                return;
            }
        }

        after = naive_date_time.and_utc().timestamp();
    }

    let dir = fs::read_dir(path.as_ref().unwrap()).unwrap();
    let parent = get_current_path(path);

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
            } else if (has_before && !has_after) && file_date <= before {
                files.push(item);
            } else if (has_after && !has_before) && file_date >= after {
                files.push(item);
            }
        }
    }

    if *&files.len() == 0 {
        println!("There are no files to sort that match the given parameters");
        return;
    }
    println!("Found {} files that are able to be sorted", &files.len());

    let mut folder = util::set_folder_name("Sorted_by_Date".to_string());

    if let Some(out_name) = args.get_one::<String>("output") {
        if !&out_name.is_empty() {
            folder = out_name.to_string();
        }
    }

    let full_path = parent.clone();
    let full_path = match util::create_folder(full_path, folder, false) {
        Ok(f) => f,
        Err(_) => return,
    };

    util::sort_files(&full_path, &files);
}
