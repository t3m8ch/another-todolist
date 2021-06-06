mod models;

use std::io;

use crate::models::{Todo, TodoStore};
use crate::cli::models::Command;

pub fn run_loop(mut todo_store: TodoStore) {
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line!");
        
        let command = parse_command(command.as_str());
        let command = match command {
            Some(cmd) => cmd,
            None => {
                println!("Command not found!");
                continue;
            }
        };

        match command {
            Command::Quit => break println!("Bye!"),
            Command::AddTodo(todo) => {
                todo_store.add(todo);
                println!("Todo added successfully!");
            },
            Command::DisplayAllTodos => display_todos(&todo_store),
            Command::DeleteTodo => delete_todo(&mut todo_store)
       };
    }
}

fn parse_command(command_str: &str) -> Option<Command> {
    match command_str.trim().to_lowercase().as_str() {
        "q" | "quit" | "exit" => Some(Command::Quit),
        "add" => Some(Command::AddTodo(enter_todo())),
        "delete" => Some(Command::DeleteTodo),
        "getall" => Some(Command::DisplayAllTodos),
        _ => None
    }
}

fn enter_todo() -> Todo {
    println!("Please enter the title of new todo");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line!");
    let title = title.trim().to_string();

    println!("Please enter the body of new todo");
    let mut body = String::new();
    io::stdin()
        .read_line(&mut body)
        .expect("Failed to read line!");
    let body = body.trim().to_string();
    
    Todo { title, body }
}

fn display_todos(todo_store: &TodoStore) {
    let todos = todo_store.get_all();
    if todos.len() == 0 {
        println!("There are no tasks, but you can add a new one");
        return
    }
    for (id, todo) in todos {
        println!("{}. {}\n{}", id, todo.title, todo.body);
    }
}

fn delete_todo(todo_store: &mut TodoStore) {
    println!("Please enter the id of the todo to be deleted");
    let id = enter_todo_id();
    match todo_store.delete(id) {
        Ok(todo) => 
            println!("The \"{}\" todo has been removed!", todo.title),
        Err(_) => println!("There is no task with this id!")
    };
}

fn enter_todo_id() -> u32 {
    loop {
        let mut id = String::new();
        io::stdin()
            .read_line(&mut id)
            .expect("Failed to read line");
        
        match id.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please enter a number!")
        }
    }
}
