extern crate dirs;
use dirs::home_dir;
use std::error::Error;
use std::{
    env,
    fs::{self, File},
};

// read list from file
fn read_from_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
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
fn add_task(filename: &str, items: &mut Vec<&str>, status: &str) {
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
    let mut formatted_data = format!("{}, {}", task, status);

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

// print list of tasks
fn list_tasks(filename: &str) {
    // creates vector using read_from_file function
    let read_data = read_from_file(filename).unwrap();

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
// remove task from list
fn remove_task(filename: &str, index: usize) {
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
fn status_toggle(filename: &str, index: usize) {
    // Reads the contents of the file and stores it in a vector of strings
    let mut read_data = read_from_file(filename).unwrap();
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
#[allow(path_statements)]
#[allow(unused_must_use)]
fn main() {
    // Get the path to the home directory
    let home_dir = match home_dir() {
        Some(path) => path,
        None => {
            println!("Error: Unable to determine home directory.");
            return;
        }
    };

    // Append config file directory to home directory
    let tasks_file_path = home_dir.join(".config").join("todo").join("tasks.txt");
    let tasks_dir_path = home_dir.join(".config").join("todo");
    // change PathBuf variables to str
    let filename = tasks_file_path.to_str().unwrap();
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
    // Get command line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the command argument is provided
    if args.len() > 1 {
        let command = &args[1];
        let mut arg: Vec<&str> = args[2..].iter().map(|s| s.as_ref()).collect();

        // Check if command is empty
        match command.is_empty() {
            true => println!("Invalid command. Use 'help' to find some info."),
            false => {
                let command = command.to_lowercase();

                // Match on the command
                match command.as_ref() {
                    "add" | "a" => {
                        match arg.is_empty() {
                            true => println!("Correct usage: todo add [task]"),
                            false => {
                                add_task(filename, &mut arg, "(false)");
                            }
                        };
                    }

                    "list" | "l" => list_tasks(filename),
                    "remove" | "r" => {
                        match arg.is_empty() {
                            true => println!("Correct usage: todo remove [task number]"),
                            false => {
                                // Convert the argument to a vector of usize (task numbers)
                                let arg_as_number: Vec<usize> =
                                    arg.iter().map(|s| s.parse::<usize>().unwrap()).collect();

                                // Get the first element of the vector (should only be one element)
                                let first_element: usize = *arg_as_number.iter().next().unwrap();
                                if first_element > 0 {
                                    // Remove the task with the specified number
                                    remove_task(filename, first_element);
                                } else {
                                    println!("Error! There is no task at the specified index.")
                                }
                            }
                        };
                    }
                    "status" | "s" => {
                        match arg.is_empty() {
                            true => println!("Correct usage: todo status [task number]"),
                            false => {
                                // Convert the argument to a vector of usize (task numbers)
                                let arg_as_number: Vec<usize> =
                                    arg.iter().map(|s| s.parse::<usize>().unwrap()).collect();

                                // Get the first element of the vector (should only be one element)
                                let first_element: usize = *arg_as_number.iter().next().unwrap();
                                if first_element > 0 {
                                    // Remove the task with the specified number
                                    status_toggle(filename, first_element);
                                } else {
                                    println!("Error! There is no task at the specified index.")
                                }
                            }
                        };
                    }
                    "help" | "h" => {
                        println!("Usage: todo [command] [options]");
                        println!(" ");
                        println!("Commands");
                        println!("add, a        Add a new task");
                        println!("remove, r     Remove a task by its number");
                        println!("list, l       List all tasks");
                        println!("status, s     Change status of a task by its number");
                        println!("help, ?       Show this help");
                        println!(" ");
                        println!("Examples");
                        println!("todo add \"Compile code\"  Add a new task");
                        println!("todo remove 2              Remove task number 2");
                        println!("todo list                  List all tasks");
                        println!("todo status 3              Change status of task number 3")
                    }
                    _ => println!("Invalid command. Use 'help' to find some info."),
                }
            }
        };
    } else {
        println!("No arguments provided. Use 'help' to find some info.");
    }
}

// UNUSED CODE

// fn append_vector(vector: &mut Vec<String>) {
//     println!("Please enter a value to write:");
//     let mut item_input = get_input();
//     item_input = item_input.trim_end().to_string();
//     vector.push(item_input);
//     println!("{:?}", vector);
// }

// fn get_input() -> String {
//     print!("Input: ");
//     io::stdout().flush().unwrap();
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();

//     input
// }
