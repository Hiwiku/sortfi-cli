use crate::constants::*;
use home;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use trash;

#[derive(Debug)]
pub enum FileAction {
    Move,
    Skip,
    Trash,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub parent_dir: PathBuf,
    pub new_path: PathBuf,
    pub action: FileAction,
}

pub fn get_home_path() -> PathBuf {
    home::home_dir().unwrap()
}

pub fn ask_user(file_name: &String, path: &PathBuf) -> FileAction {
    loop {
        print!(
            "{BOLD_BLUE}Move{DEFAULT_COLOR} {BOLD}{}{DEFAULT_COLOR} {BOLD_BLUE}to{DEFAULT_COLOR} {BOLD}{:?}{DEFAULT_COLOR}{BOLD_BLUE}?{DEFAULT_COLOR} {BOLD}[Y/n/t]{DEFAULT_COLOR} ",
            file_name, path
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "Y" | "y" | "" => return FileAction::Move,
            "N" | "n" => return FileAction::Skip,
            "T" | "t" => return FileAction::Trash,
            _ => println!("Unknown command!"),
        }
    }
}

pub fn get_target_folder(file_type: Option<&OsStr>) -> PathBuf {
    let file_type = file_type.unwrap().to_str().unwrap();

    if crate::constants::PICTURE_EXTENSIONS.contains(&file_type) {
        PathBuf::from(crate::constants::PICTURE_PATH)
    } else if crate::constants::MUSIC_EXTENSIONS.contains(&file_type) {
        PathBuf::from(crate::constants::MUSIC_PATH)
    } else if crate::constants::VIDEO_EXTENSIONS.contains(&file_type) {
        PathBuf::from(crate::constants::VIDEO_PATH)
    } else {
        PathBuf::from(crate::constants::DOCUMENT_PATH)
    }
}

pub fn analysis(path: PathBuf, home_path: PathBuf) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap().path();
        if !entry.is_file() {
            continue;
        }

        let name = entry.file_name().unwrap().to_string_lossy().into_owned();
        if name.starts_with('.') || entry.extension().is_none() {
            continue;
        }

        let file_type = entry.extension();
        let target_folder = get_target_folder(file_type);

        let mut new_path = home_path.clone();
        new_path.push(&target_folder);
        let parent_dir = new_path.clone();
        new_path.push(&name);

        let action = ask_user(&name, &target_folder);

        let file = File {
            name: name,
            path: entry,
            parent_dir: parent_dir,
            new_path: new_path,
            action: action,
        };
        files.push(file)
    }
    files
}

pub fn run_actions(files: Vec<File>) {
    for file in files {
        match file.action {
            FileAction::Move => {
                fs::create_dir_all(&file.parent_dir).unwrap();
                fs::rename(&file.path, &file.new_path).unwrap();
            }
            FileAction::Skip => continue,
            FileAction::Trash => {
                trash::delete(&file.path).unwrap();
            }
        }
    }
    println!("{}Done!", BOLD_GREEN)
}
