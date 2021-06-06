use crate::models::Todo;

pub enum Command {
    Quit,
    AddTodo(Todo),
    DisplayAllTodos,
    DeleteTodo
}
