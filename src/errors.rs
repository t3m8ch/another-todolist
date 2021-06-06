extern crate custom_error;
use custom_error::custom_error;

custom_error!{pub DeleteTodoError
    IdNotFound = "There is no task with this id"
}