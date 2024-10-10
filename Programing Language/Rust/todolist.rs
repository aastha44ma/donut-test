// first run this in terminal//
cargo new todo_list
cd todo_list
//code//
use std::collections::HashMap;
use std::io::{self, Write};

struct TodoList {
    tasks: HashMap<u32, String>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, task: String) -> u32 {
        let id = self.next_id;
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }

    fn remove_task(&mut self, id: u32) -> Option<String> {
        self.tasks.remove(&id)
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            for (id, task) in &self.tasks {
                println!("{}: {}", id, task);
            }
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!("\nTo-Do List:");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Remove task");
        println!("4. Exit");
        print!("Choose an option: ");
        
        io::stdout().flush().unwrap();  // Ensure prompt is printed before input

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        match choice.trim() {
            "1" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read line");
                let task_id = todo_list.add_task(task.trim().to_string());
                println!("Task added with ID: {}", task_id);
            }
            "2" => {
                todo_list.list_tasks();
            }
            "3" => {
                print!("Enter task ID to remove: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin()
                    .read_line(&mut id_str)
                    .expect("Failed to read line");
                let id: u32 = id_str.trim().parse().unwrap_or(0);
                match todo_list.remove_task(id) {
                    Some(task) => println!("Removed task: {}", task),
                    None => println!("No task found with ID: {}", id),
                }
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}
