use rar::read_from_file;
use std::fs;
// print list of tasks
pub fn list_tasks(filename: &str) {
    // creates vector using read_from_file function
    let read_data = read_from_file(filename).unwrap();

    // check if task list is empty
    if read_data.is_empty() {
        println!("The list is empty!");
        return;
    } //prints items and adds numeration to them
    println!("Task list:");
    for (i, item) in read_data.iter().enumerate() {
        let item = item.replace(", (true)", " ✅ finished!");
        let item = item.replace(", (false)", " ❎ TODO");
        println!("{}. {}", i + 1, item);
    }
}
// edit task from list
pub fn edit_task(filename: &str, items: &mut Vec<&str>) {
    // Reads the contents of the file and stores it in a vector of strings
    let mut read_data = read_from_file(filename).unwrap();

    // Get the first index from vector and change it to a number
    let index = items.first();
    let index: usize = match index {
        Some(s) => s.parse().unwrap(),
        None => 0,
    };
    // Remove first index from vector
    if index > 0 {
        items.remove(0);
    } else {
        println!("Error! There is no task at the specified index.");
        return;
    }

    if index <= read_data.len() {
        let mut task = items.join(" ");
        if read_data[index - 1].ends_with(", (true)") {
            task = task + ", (true)";
            read_data[index - 1] = task;
        } else if read_data[index - 1].ends_with(", (false)") {
            task = task + ", (false)";
            read_data[index - 1] = task;
        }
        // Joins the vector into a single string with \n separating elements
        let data = read_data.join("\n");

        // Write the modified data back to the file
        match fs::write(filename, data) {
            Ok(()) => println!("Task edited"),
            Err(error) => println!("Error occurred while writing to file: {}", error),
        }
    } else {
        println!("Error! There is no task at the specified index.");
    }
}

pub fn status_toggle(filename: &str, index: usize) {
    // Reads the contents of the file and stores it in a vector of strings
    let mut read_data = read_from_file(filename).unwrap();

    // Check if given index is on list
    if index <= read_data.len() {
        if read_data[index - 1].ends_with(", (true)") {
            read_data[index - 1] = read_data[index - 1].replace(", (true)", ", (false)");
            // Joins the vector into a single string with \n separating elements
            let data = read_data.join("\n");

            // Write the modified data back to the file
            match fs::write(filename, data) {
                Ok(()) => println!("Task status set to unfinished!"),
                Err(error) => println!("Error occurred while writing to file: {}", error),
            };
        } else if read_data[index - 1].ends_with(", (false)") {
            read_data[index - 1] = read_data[index - 1].replace(", (false)", ", (true)");

            // Joins the vector into a single string with \n separating elements
            let data = read_data.join("\n");

            // Write the modified data back to the file
            match fs::write(filename, data) {
                Ok(()) => println!("Task status set to completed!"),
                Err(error) => println!("Error occurred while writing to file: {}", error),
            };
        }
    } else {
        println!("Error! There is no task at the specified index.");
    }
}
