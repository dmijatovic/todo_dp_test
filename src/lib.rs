#[macro_use]
extern crate log;

mod db;

pub use db::todo_item::{NewTodoItem, TodoItem};
pub use db::todo_list::{NewTodoList, TodoList, UpdateTodoList};
pub use db::{create_pool, Pool};
pub use db::{todo_item, todo_list};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
