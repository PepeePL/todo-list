mod args;
mod help;
mod les;
mod paths;
mod rar;
use paths::{create_filepath, get_filename};
extern crate dirs;
use args::command;

fn main() {
    let binding = get_filename().unwrap();
    let filename = binding.as_str();
    create_filepath(filename);
    command(filename);
}
