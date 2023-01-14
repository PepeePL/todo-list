pub fn print_help() {
    println!("Usage: todo [command] [options]");
    println!(" ");
    println!("Commands");
    println!("add, a <task>                       Add a new task");
    println!("remove, r <task number|all>         Remove a task by its number or all tasks.");
    println!("list, l                             List all tasks");
    println!("status, s <task number>             Change status of a task by its number");
    println!("edit, e <task number> <new task>    Edit task by its number");
    println!("help, h                             Show this help");
    println!(" ");
    println!("Examples");
    println!("todo add \"Compile code\"    Add a new task");
    println!("todo remove 2              Remove task number 2");
    println!("todo list                  List all tasks");
    println!("todo status 3              Change status of task number 3")
}
