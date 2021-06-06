use std::collections::HashMap;

use crate::errors::DeleteTodoError;

#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub body: String
}

pub struct TodoStore {
    todos: HashMap<u32, Todo>
}

impl TodoStore {
    pub fn new() -> TodoStore {
        TodoStore { todos: HashMap::new() }
    }
    
    pub fn add(&mut self, todo: Todo) {
        let id = match self.todos.keys().max() {
            Some(n) => n.clone() + 1,
            None => 0
        };
        self.todos.insert(id, todo);
    }

    pub fn delete(&mut self, id: u32) -> Result<Todo, DeleteTodoError> {
        match self.todos.remove(&id) {
            Some(todo) => Ok(todo),
            None => Err(DeleteTodoError::IdNotFound)
        }
    }
    
    pub fn get_all(&self) -> &HashMap<u32, Todo> {
        &self.todos
    }
}
