use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::{Arg, ArgMatches, Command};

use crate::util;
use crate::util::path::get_path;

pub fn cli() -> Command {
    Command::new("type")
        .about("Sort all or select files by file type")
        .args([
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

    if let Some(_) = args.get_one::<String>("output") {
        println!("NOTE: Setting a custom output folder is currently not possible when sorting by file type");
    }

    let dir = fs::read_dir(path.as_ref().unwrap()).unwrap();
    let paths_parent = path.as_ref().unwrap().display().to_string(); // As a String
    let parent = path.unwrap(); // PathBuf
    println!("CURRENT PATH: {}", &paths_parent);

    let mut files: Vec<DirEntry> = vec![];
    let mut file_types: Vec<String> = vec![];

    if let Some(ftype) = args.get_one::<String>("type") {
        let folder = ftype.to_string();

        let full_path = parent.clone();
        let full_path = match util::create_folder(full_path, folder, true) {
            Ok(f) => f,
            Err(_) => return,
        };

        for item in dir {
            let item = item.unwrap();
            let md = item.metadata().unwrap();

            if md.is_file() {
                let filename = &item.file_name();
                let extension = Path::new(filename).extension().and_then(OsStr::to_str);
                let ext: String;
                match extension {
                    Some(f) => ext = f.to_string(),
                    None => continue,
                };
                if ext == ftype.to_string() {
                    files.push(item);
                }
            } else {
                continue;
            }
        }

        util::sort_files(&full_path, &files);

        return;
    }

    for item in dir {
        let item = item.unwrap();
        let md = item.metadata().unwrap();
        if md.is_file() {
            let filename = &item.file_name();
            files.push(item);

            let f_type = Path::new(filename).extension().and_then(OsStr::to_str);
            let ff: String;
            match f_type {
                Some(f) => {
                    ff = f.to_string();
                    if !file_types.contains(&ff) {
                        file_types.push(ff);
                    }
                }
                None => {}
            };
        } else {
            // Ignore directories (for now)
            continue;
        }
    }

    if *&files.len() == 0 {
        println!("There are no files to sort");
        return;
    }
    println!(
        "Found {} files with {} unique file types",
        &files.len(),
        &file_types.len()
    );

    // Create file type paths
    for file_type in &file_types {
        let full_path = parent.clone();
        _ = util::create_folder(full_path, file_type.to_string(), true);
    }

    // TODO: Check if output is specified and warn user that it will not be used for
    // sorting all files by their file type(s)

    let mut files_sorted: f64 = 0.0;
    let start = Instant::now();
    // TODO: maybe have progress bar
    let mut stdout = stdout();
    for (idx, file) in files.iter().enumerate() {
        let done = idx as f64 / *&files.len() as f64;

        let fname = &file.file_name();
        let ext = Path::new(fname).extension().and_then(OsStr::to_str);
        let ff: String;
        match ext {
            Some(f) => ff = f.to_string(),
            None => continue,
        };

        // get original directory and navigate to file type directories
        let full_path = parent.clone().join(ff);

        let f = fs::rename(file.path(), full_path.join(file.file_name()));
        match f {
            Ok(_) => files_sorted += 1.0,
            Err(error) => println!("There was a problem opening the file:\n{:?}", error),
        };

        print!("\rProcessing {:.1}%", done * 100.0);
        stdout.flush().unwrap();
    }

    let duration = start.elapsed();
    stdout.flush().unwrap();
    print!("\rProcessed 100%   \n");
    println!("Time taken: {:?}", duration);
    println!(
        "Sorted {}/{} files into folders",
        &files_sorted,
        &files.len()
    );
}
