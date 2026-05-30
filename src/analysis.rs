use crate::constants::*;
use std::fs;
use std::io::{self, Write};

pub fn analysis(path: &str) -> io::Result<()> {
    let iterator = fs::read_dir(path)?;
    for entry in iterator {
        let entry = entry?;
        let filename = entry.file_name();
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(ext_os) = path.extension() {
            if let Some(ext_str) = ext_os.to_str() {
                let ext_lower = ext_str.to_lowercase();
                let target_folder: &str;
                if PICTURE_EXTENSIONS.contains(&ext_lower.as_str()) {
                    target_folder = "Pictures";
                } else if MUSIC_EXTENSIONS.contains(&ext_lower.as_str()) {
                    target_folder = "Music";
                } else if DOCUMENT_EXTENSIONS.contains(&ext_lower.as_str()) {
                    target_folder = "Documents";
                } else if VIDEO_EXTENSIONS.contains(&ext_lower.as_str()) {
                    target_folder = "Videos";
                } else {
                    continue;
                }
                loop {
                    print!("Move {:?} -> ~/{target_folder} [Y/n]? ", filename);
                    io::stdout().flush().expect("Error!");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Error!");
                    let input = input.trim();

                    if input == "" || input == "Y" || input == "y" {
                        if let Ok(home_str) = std::env::var("HOME") {
                            let mut home_path = std::path::PathBuf::from(home_str);
                            home_path.push(target_folder);
                            let _ = std::fs::create_dir_all(&home_path);
                            let mut new_path = home_path;
                            new_path.push(&filename);
                            let _ = std::fs::rename(&path, &new_path);
                        }
                        break;
                    } else if input == "n" || input == "N" {
                        break;
                    } else {
                        println!("Invalid command!\n");
                    }
                }
            }
        }
    }
    println!("Done!");
    Ok(())
}
