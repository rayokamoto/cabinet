use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::{Arg, ArgMatches, Command};

use crate::path::get_path;
use crate::utils;

pub fn cli() -> Command {
    Command::new("type")
        .about("Sort files by file type")
        .alias("T")
        .args([
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

    // TODO: Deal with symlinks

    let dir = fs::read_dir(path.as_ref().unwrap()).unwrap();
    let paths_parent = path.as_ref().unwrap().display().to_string(); // As a String
    let parent = path.unwrap(); // PathBuf
    println!("CURRENT PATH: {}", &paths_parent);

    let mut files: Vec<DirEntry> = vec![];
    let mut file_types: Vec<String> = vec![];

    for item in dir {
        // unwrap item to get Ok(item) i.e. DirEntry
        let item = item.unwrap();
        //println!("FILE: {:?}", &item.path());
        // get metadata for item then unwrap to get Ok() value instead of Err()
        let md = item.metadata().unwrap();
        if md.is_file() {
            let filename = &item.file_name();
            files.push(item);

            // Will panic when it encounters file with no extension.
            //let f_type = Path::new(filename).extension().and_then(OsStr::to_str).unwrap().to_string();
            
            // One method
            //let f_type = Path::new(filename).extension().and_then(OsStr::to_str);
            //let ff: String;
            //if let Some(f) = f_type {
            //    ff = f.to_string();
            //    if !file_types.contains(&ff) {
            //        file_types.push(ff);
            //    }
            //}
            //else {
            //    println!("PANICKED!!!");
            //    return;
            //}

            
            //if !file_types.contains(&ff) {
            //    file_types.push(ff);
            //}

            let f_type = Path::new(filename).extension().and_then(OsStr::to_str);
            let ff: String;
            match f_type {
                Some(f) => {
                    ff = f.to_string();
                    if !file_types.contains(&ff) {
                        file_types.push(ff);
                    }
                },
                None => {}
            };
        }
        else {
            // Ignore directories (for now)
            continue;
        }
    }

    // Not that this will not run if there are files with no file extensions
    if *&files.len() == 0 { // dereference, otherwise &usize will be compared to int
        println!("There are no files to sort");
        return;
    }
    println!("Found {} files with {} unique file types", &files.len(), &file_types.len());

    // Create file type paths 
    for file_type in &file_types {
        let mut full_path = parent.clone(); 
        // cannot reference (&) since we would be pushing to that reference below:
        full_path.push(&file_type);
        utils::create_folder(&full_path, &file_type)
    }

    let mut files_sorted: f64 = 0.0;
    let start = Instant::now();
    // TODO: maybe have progress bar
    let mut stdout = stdout();
    for (idx, file) in files.iter().enumerate() {
        let done = idx as f64 / *&files.len() as f64;
        // Get file extension to sort into folder for that file extension

        //let ext = Path::new(&file.file_name()).extension().and_then(OsStr::to_str).unwrap().to_string();
        let fname = &file.file_name();
        // If Path::new(&file.file_name()) is used, rustc(E0716) is raised. 
        // It talks about how value is dropped when ext is matched, perhaps &file dropped? 
        // Bug?
        let ext = Path::new(fname).extension().and_then(OsStr::to_str);
        let ff: String;
        match ext {
            Some(f) => {
                ff = f.to_string();
            },
            None => {continue;}
        };

        
        // get original directory and navigate to file type directories
        let full_path = parent.clone().join(ff);

        //println!("FROM: {:?} --TO: {:?}", &file.path(), &full_path.join(file.file_name()));
        let f = fs::rename(file.path(), full_path.join(file.file_name()));
        match f {
            Ok(_) => files_sorted += 1.0,
            Err(error) => println!("There was a problem opening the file:\n{:?}", error)
        };
        
        print!("\rProcessing {:.1}%", done * 100.0);
        stdout.flush().unwrap();
        //sleep(Duration::from_millis(10));

    }
    let duration = start.elapsed();
    stdout.flush().unwrap();
    // \t doesn't seem to actually work in clearing everything - you get "Processed 100%9%"
    print!("\rProcessed 100%   \n"); 
    println!("Time taken: {:?}", duration);
    println!("Sorted {}/{} files into folders", &files_sorted, &files.len());

    //println!("{:?}", &files);
    //println!("{:?}", &file_types);
}
