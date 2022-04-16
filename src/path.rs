use std::path::{Path, PathBuf};

use dirs;


#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentType {
    None,
    Path,
    Template,
    //Help
}

/// Get filepath object
/// 
/// Only called when it is a valid path option -p or -t
pub fn get_path(path: &String, path_type: ArgumentType) -> Option<PathBuf> {
    let mut path_ref: Option<PathBuf> = None;

    if path_type == ArgumentType::Template {
        let path = &path.to_lowercase()[..];
        match path {
            "documents" => path_ref = dirs::document_dir(),
            "desktop" => path_ref = dirs::desktop_dir(),
            "downloads" => path_ref = dirs::download_dir(),
            "home" => path_ref = dirs::home_dir(),
            "music" | "audio" => path_ref = dirs::audio_dir(),
            "pictures" => path_ref = dirs::picture_dir(),
            "videos" | "movies" => path_ref = dirs::video_dir(),
            _ => println!("The template '{}' does not exist.", &path)
        }
    }
    else if path_type == ArgumentType::Path {
        if Path::new(path).exists() {
            path_ref = Some(PathBuf::from(path));
        }
        else {
            println!("Directory \"{}\" either does not exist or this program is missing permissions to access it.", &path);
        }
    }
    else {
        // This should never be called.
        println!("Not a valid path type.");
    }

    path_ref
}
