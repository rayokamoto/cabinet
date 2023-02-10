use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::UNIX_EPOCH;

use chrono::NaiveDateTime;
use clap::{Arg, ArgMatches, Command};
use regex::Regex;

use crate::util;
use crate::util::path::get_path;

pub fn cli() -> Command {
    Command::new("multisort")
        .about("Sort files using multiple file attributes")
        .alias("sort")
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
            Arg::new("includes")
                .short('I')
                .long("includes")
                .value_name("match")
                .help("File name includes...")
                .action(clap::ArgAction::Set),
            Arg::new("excludes")
                .short('E')
                .long("excludes")
                .value_name("match")
                .help("File name excludes...")
                .action(clap::ArgAction::Set),
            Arg::new("min")
                .short('m')
                .long("min")
                .value_name("size")
                .help("Get files that are GREATER THAN the specified size (in KB)")
                .action(clap::ArgAction::Set),
            Arg::new("max")
                .short('M')
                .long("max")
                .value_name("size")
                .help("Get files that are LESS THAN the specified size (in KB)")
                .action(clap::ArgAction::Set),
            Arg::new("type")
                .short('T')
                .long("type")
                .value_name("file-type")
                .help("Sort files according to the specific file type")
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
                .required(true),
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

    // Date pattern for YYYY-MM-DD
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let mut date_after: Option<String> = None;
    let mut date_before: Option<String> = None;
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

    let mut include_pattern: Option<String> = None;
    let mut exclude_pattern: Option<String> = None;
    if let Some(incl) = args.get_one::<String>("includes") {
        include_pattern = Some(incl.to_string());
    }
    if let Some(excl) = args.get_one::<String>("excludes") {
        exclude_pattern = Some(excl.to_string());
    }

    let mut size_min: Option<String> = None;
    let mut size_max: Option<String> = None;
    if let Some(min) = args.get_one::<String>("min") {
        size_min = Some(min.to_string());
    }
    if let Some(max) = args.get_one::<String>("max") {
        size_max = Some(max.to_string());
    }

    let mut file_type: Option<String> = None;
    if let Some(ftype) = args.get_one::<String>("type") {
        file_type = Some(ftype.to_string());
    }

    // No options were provided
    if size_min == None
        && size_max == None
        && include_pattern == None
        && exclude_pattern == None
        && date_before == None
        && date_after == None
        && file_type == None
    {
        println!("ERROR: At least one option must be provided");
        return;
    }

    let dir = fs::read_dir(path.as_ref().unwrap()).unwrap();
    let paths_parent = path.as_ref().unwrap().display().to_string();
    let parent = path.unwrap();
    println!("CURRENT PATH: {}", &paths_parent);

    let mut files: Vec<Rc<DirEntry>> = vec![];
    let mut sort_files = false;

    for item in dir {
        let item = item.unwrap();
        //files.push(item.into());
        files.push(Rc::new(item));
    }

    // Sort by name

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

    let mut new_files: Vec<Rc<DirEntry>> = vec![];

    for item in &files {
        let md = item.metadata().unwrap();

        let filename = &item.file_name(); // gets the name (no file extension)
        let f = OsStr::to_str(&filename).unwrap();

        if md.is_file() {
            if (has_include && has_exclude) && (f.contains(&include) && !f.contains(&exclude)) {
                new_files.push(Rc::clone(&item));
            } else if (has_include && !has_exclude) && f.contains(&include) {
                new_files.push(Rc::clone(&item));
            }
            // Only add file if it DOESN'T include the pattern (since it is exclude pattern)
            else if (has_exclude && !has_include) && !f.contains(&exclude) {
                new_files.push(Rc::clone(&item));
            }
        }
    }

    if !&new_files.is_empty() {
        files = new_files;
        sort_files = true;
    } else if new_files.is_empty() && (has_include || has_exclude) {
        files = vec![];
    }

    // Sort by date

    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

    let mut has_before = false;
    let mut has_after = false;
    let mut before: i64 = 0;
    let mut after: i64 = 0;

    if date_before != None {
        has_before = true;
        let d = &date_before.as_ref().unwrap()[..];
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
        before = naive_date_time.timestamp();
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

        after = naive_date_time.timestamp();
    }

    let mut new_files: Vec<Rc<DirEntry>> = vec![];
    for item in &files {
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
                new_files.push(Rc::clone(&item));
            } else if (has_before && !has_after) && file_date <= before {
                new_files.push(Rc::clone(&item));
            } else if (has_after && !has_before) && file_date >= after {
                new_files.push(Rc::clone(&item));
            }
        }
    }

    if !&new_files.is_empty() {
        files = new_files;
        sort_files = true;
    } else if new_files.is_empty() && (has_before || has_after) {
        files = vec![];
    }

    // Sort by size

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

    let mut new_files: Vec<Rc<DirEntry>> = vec![];
    for item in &files {
        let md = item.metadata().unwrap();

        if md.is_file() {
            let file_size = &md.len() / 1000; // Convert bytes to kilobytes

            //println!("filesize:{} - min-check:{:?}, max-check:{:?}", &file_size, file_size >= min, file_size <= max);
            if (has_min && has_max) && (file_size >= min && file_size <= max) {
                new_files.push(Rc::clone(&item));
            } else if (has_min && !has_max) && file_size >= min {
                new_files.push(Rc::clone(&item));
            } else if (has_max && !has_min) && file_size <= max {
                new_files.push(Rc::clone(&item));
            }
        }
    }

    if !&new_files.is_empty() {
        files = new_files;
        sort_files = true;
    } else if new_files.is_empty() && (has_min || has_max) {
        files = vec![];
    }

    if file_type != None {
        let mut new_files: Vec<Rc<DirEntry>> = vec![];
        for item in &files {
            let md = item.metadata().unwrap();

            if md.is_file() {
                let filename = &item.file_name();
                let extension = Path::new(filename).extension().and_then(OsStr::to_str);
                let ext: String;
                match extension {
                    Some(f) => ext = f.to_string(),
                    None => continue,
                };
                if ext == file_type.clone().unwrap() {
                    new_files.push(Rc::clone(&item));
                }
            }
        }

        if !&new_files.is_empty() {
            files = new_files;
            sort_files = true;
        } else if new_files.is_empty() && file_type != None {
            files = vec![];
        }
    }

    if !sort_files {
        files = vec![];
    }

    if *&files.len() == 0 {
        println!("There are no files to sort that match the given parameters");
        return;
    }
    println!("Found {} files that are able to be sorted", &files.len());

    let mut folder = util::set_folder_name("Multisort".to_string());

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

    util::sort_files_rc(&full_path, &files);
}
