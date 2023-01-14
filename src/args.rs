use help::print_help;
use les::*;
use rar::*;
use std::env;
pub fn command(filename: &str) {
    // Get command line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the command argument is provided
    if args.len() == 1 {
        println!("No arguments provided. Use 'help' to find some info.");
    } else {
        // Break provided arguments into two pieces
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
                                add_task(filename, &mut arg);
                            }
                        };
                    }

                    "list" | "l" => list_tasks(filename),
                    "remove" | "r" => {
                        match arg.is_empty() {
                            true => println!("Correct usage: todo remove [task number|all]"),
                            false => {
                                if arg.contains(&"all") {
                                    remove_all_tasks(filename)
                                } else {
                                    // Convert the argument to a vector of usize (task numbers)
                                    let arg_as_number: Vec<usize> =
                                        arg.iter().map(|s| s.parse::<usize>().unwrap()).collect();

                                    // Get the first element of the vector (should only be one element)
                                    let first_element: usize =
                                        *arg_as_number.iter().next().unwrap();
                                    if first_element > 0 {
                                        // Remove the task with the specified number
                                        remove_task(filename, first_element);
                                    } else {
                                        println!("Error! There is no task at the specified index.")
                                    }
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
                    "edit" | "e" => edit_task(filename, &mut arg),
                    "help" | "h" => print_help(),
                    _ => println!("Invalid command. Use 'help' to find some info."),
                }
            }
        };
    }
}
