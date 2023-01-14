extern crate dirs;

use std::{
    fmt::Error,
    fs::{self, File},
    path::PathBuf,
};

use self::dirs::home_dir;
// Get the path to the home directory

fn get_home_dir() -> Result<PathBuf, Error> {
    let home_dir = match home_dir() {
        Some(path) => path,
        None => {
            println!("Error: Unable to determine home directory.");
            return Err(Error);
        }
    };
    return Ok(home_dir);
}
pub fn get_filename() -> Result<String, Error> {
    // Append config file directory to home directory
    let tasks_file_path = get_home_dir()
        .unwrap()
        .join(".config")
        .join("todo")
        .join("tasks.txt");

    // change PathBuf variables to str
    let filename = tasks_file_path.to_str().unwrap().to_string();
    return Ok(filename);
}
#[allow(unused_must_use)]
#[allow(path_statements)]
pub fn create_filepath(filename: &str) {
    // Append config file directory to home directory
    let tasks_dir_path = get_home_dir().unwrap().join(".config").join("todo");

    // change PathBuf variables to str
    let filepath = tasks_dir_path.to_str().unwrap();

    // Check if the file at filepath exists
    if let Err(_metadata) = fs::metadata(filepath) {
        // If the file doesn't exist, try to create the directory specified by filepath
        let _file = match fs::create_dir_all(filepath) {
            // If the directory was created successfully, create the file
            Ok(file) => {
                file;
                File::create(filename);
            }
            // If there was an error creating the directory, panic and display the error message
            Err(error) => panic!("Error creating file: {}", error),
        };
    }
}
