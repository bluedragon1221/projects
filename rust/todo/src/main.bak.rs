use inquire::{Select, Text};
use std::fmt::Display;
use todo::{TodoError, TodoItem, TodoItemState, TodoList};

use lazy_static::lazy_static;

pub fn menu<T: Display>(items: Vec<T>) -> Result<T, TodoError> {
    Ok(Select::new("", items).prompt()?)
}

pub fn text(prompt: &str) -> Result<String, TodoError> {
    Ok(Text::new(prompt).prompt()?)
}

fn select_todo_index(todo_list: &TodoList) -> Result<usize, TodoError> {
    Ok(todo_list.get_index_of(menu(todo_list.get_items())?)?)
}

fn cycle_item(todo_list: &mut TodoList) -> Result<(), TodoError> {
    todo_list
        .mut_by_index(select_todo_index(&todo_list)?)
        .toggle();
    Ok(())
}

#[derive(Clone)]
enum Actions {
    Mark(TodoItemState),
    Del,
    Add,
    List,
}

impl Actions {
    fn do_action(&self, todo_list: &mut TodoList) -> Result<(), TodoError> {
        match self {
            Self::Mark(a) => {
                let index = select_todo_index(&todo_list)?;
                todo_list.mut_by_index(index).set_state(a.clone());
                Ok(())
            }
            Self::Del => {
                let index = select_todo_index(&todo_list)?;
                todo_list.del_item(index);
                Ok(())
            }
            Self::Add => {
                let item_name = text("What do you need to do?")?;
                todo_list.add_item(TodoItem::new(&item_name));
                Ok(())
            }
            Self::List => Ok(()),
        }
    }
}

lazy_static! {
    static ref ALL_ACTIONS: Vec<Actions> = vec![
        Actions::Mark(TodoItemState::Todo),
        Actions::Mark(TodoItemState::InProgress),
        Actions::Mark(TodoItemState::Done),
        Actions::Del,
        Actions::Add,
        Actions::List,
    ];
}

impl Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Mark(TodoItemState::Todo) => "Mark item as Todo",
                Self::Mark(TodoItemState::InProgress) => "Mark item as In Progress",
                Self::Mark(TodoItemState::Done) => "Mark item as Done",
                Self::Del => "Delete an Item",
                Self::Add => "Add an Item",
                Self::List => "Print the Todo list",
            }
        )
    }
}

fn main() -> Result<(), TodoError> {
    let mut todo_list = TodoList::new("Some things that I need to do");

    todo_list.add_item(TodoItem::new("Take out the trash"));
    todo_list.add_item(TodoItem::new("Do homework"));

    loop {
        println!("{}---", todo_list.format()?);
        menu(ALL_ACTIONS.to_vec())?.do_action(&mut todo_list)?;
        println!("{}", todo_list.format()?);
    }
}

// fn main() -> Result<(), TodoError> {
//     let mut todo_list = TodoList::new("Some things that I need to do");

//     todo_list.add_item(TodoItem::new("Take out the trash"));
//     todo_list.add_item(TodoItem::new("Do homework"));

//     println!("{}", todo_list.format()?);

//     todo_list.del_item(0);

//     println!("{}", todo_list.format()?);

//     todo_list.mut_by_index(0).toggle();
//     todo_list.add_item(TodoItem::new("And another thing"));
//     todo_list.mut_by_index(1).set_state(TodoItemState::Done);

//     println!("{}", todo_list.format()?);

//     Ok(())
// }
