use crate::utils::printing;
use dirs;
use std::fs;
use std::path::PathBuf;

pub fn create_dir(path: &PathBuf) {
    if path.exists() {
        printing::info(format!(
            "A directory already exists at {}.",
            path.to_string_lossy()
        ))
    } else {
        printing::progress(format!("Creating directory at {}.", path.to_string_lossy()));
        match fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn remove_dir(path: &PathBuf) {
    if path.exists() {
        printing::progress(format!("Deleting directory at {}.", path.to_string_lossy()));
        match fs::remove_dir_all(&path) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    } else {
        printing::info(format!(
            "A directory doesn't exist at {}.",
            path.to_string_lossy()
        ));
    }
}

pub fn create_file(path: &PathBuf, contents: &str) {
    if path.exists() {
        printing::info(format!(
            "A file already exists at {}.",
            path.to_string_lossy()
        ))
    } else {
        printing::progress(format!("Creating file at {}.", path.to_string_lossy()));
        match fs::File::create(&path) {
            Ok(_) => {
                match fs::write(&path, &contents) {
                    Ok(_) => (),
                    Err(error) => panic!("There was a problem: {:?}", error),
                };
            }
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn file_stem(file: PathBuf) -> std::string::String {
    match file.file_stem() {
        Some(file_stem) => match file_stem.to_str() {
            Some(string) => string.to_string(),
            None => panic!("There was a problem."),
        },
        None => panic!("Cannot find the file stem for {}.", file.to_string_lossy()),
    }
}

pub fn vim_plugins() -> PathBuf {
    home()
        .join(".vim")
        .join("pack")
        .join("plugins")
        .join("start")
}

pub fn repos() -> PathBuf {
    home().join("repos")
}

pub fn notes() -> PathBuf {
    project().join("src/notes")
}

pub fn public() -> PathBuf {
    project().join("public")
}

pub fn cname() -> PathBuf {
    public().join("CNAME")
}

fn home() -> PathBuf {
    match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    }
}

fn project() -> PathBuf {
    repos().join("trevordmiller")
}
