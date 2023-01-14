use std::{error::Error, fs};

// read list from file
pub fn read_from_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Reads the contents of the file into a string
    let data = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(error) => return Err(Box::new(error)),
    };

    // Splits the contents of the file into a vector of strings, one element per line
    let items: Vec<String> = data.lines().map(|s| s.to_string()).collect();

    // Returns the vector wrapped in a Result object
    Ok(items)
}

// add task to the list
pub fn add_task(filename: &str, items: &mut Vec<&str>) {
    // Reads the contents of the file and stores it in a vector of strings
    let mut read_data = read_from_file(filename).unwrap();

    // Joins the task into a single string
    let mut task = items.join(" ");

    // If string starts or ends with spaces, remove them
    if task.starts_with(" ") {
        task = task.trim_start().to_string();
    }
    if task.ends_with(" ") {
        task = task.trim_end().to_string();
    }

    // Formats the task string and the status integer into a single string separated by a comma
    let mut formatted_data = format!("{}, (false)", task);

    // Adds the formatted string to the vector
    read_data.push(formatted_data);

    // Joins the vector into a single string with \n separating elements
    formatted_data = read_data.join("\n");

    // Writes the formatted string to the file
    match fs::write(filename, formatted_data) {
        Ok(()) => println!("Task added successfully"),
        Err(error) => println!("Error occurred while writing to file: {}", error),
    }
}

// remove task from list
pub fn remove_task(filename: &str, index: usize) {
    // Reads the contents of the file and stores it in a vector of strings
    let mut read_data = read_from_file(filename).unwrap();

    if index <= read_data.len() {
        // Remove the task at the specified index from the vector
        read_data.remove(index - 1);
        // Joins the vector into a single string with \n separating elements
        let data = read_data.join("\n");

        // Write the modified data back to the file
        match fs::write(filename, data) {
            Ok(()) => println!("Task removed"),
            Err(error) => println!("Error occurred while writing to file: {}", error),
        }
    } else {
        println!("Error! There is no task at the specified index.");
    }
}

// remove all tasks from list
pub fn remove_all_tasks(filename: &str) {
    // Reads the contents of the file and stores it in a vector of strings
    let mut read_data = read_from_file(filename).unwrap();

    if read_data.len() > 0 {
        // Remove all tasks from the vector
        read_data.clear();

        // Joins the vector into a single string with \n separating elements
        let data = read_data.join("\n");

        // Write the modified data back to the file
        match fs::write(filename, data) {
            Ok(()) => println!("All tasks removed"),
            Err(error) => println!("Error occurred while writing to file: {}", error),
        }
    } else {
        println!("Error! The list is empty.");
    }
}
