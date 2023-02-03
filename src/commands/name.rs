use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

use crate::util;
use crate::util::path::get_path;

pub fn cli() -> Command {
    Command::new("name")
        .about("Sort files by file name")
        .args([
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

    let mut include_pattern: Option<String> = None;
    let mut exclude_pattern: Option<String> = None;

    if let Some(incl) = args.get_one::<String>("includes") {
        include_pattern = Some(incl.to_string());
    }
    if let Some(excl) = args.get_one::<String>("excludes") {
        exclude_pattern = Some(excl.to_string());
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
            } else if (has_include && !has_exclude) && f.contains(&include) {
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
    let mut folder = util::set_folder_name("Sorted_by_Name".to_string());

    if let Some(out_name) = args.get_one::<String>("output") {
        if !&out_name.is_empty() {
            folder = util::set_folder_name(out_name.to_string());
        }
    }

    let full_path = parent.clone();
    let full_path = match util::create_folder(full_path, folder) {
        Ok(f) => f,
        Err(_) => return,
    };

    util::sort_files(&full_path, &files);
}
