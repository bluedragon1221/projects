use std::slice::Iter;

use fuzzy_match::fuzzy_match;
use fuzzy_matcher::skim::SkimMatcherV2;
use savefile::prelude::*;
use thiserror;

#[macro_use]
extern crate savefile_derive;

#[derive(Debug, thiserror::Error)]
enum TodoError {
    #[error("Error when saving to file")]
    SaveError(#[from] savefile::SavefileError),
    #[error("It looks like you haven't set a save file yet")]
    NoSaveFile,
}

#[derive(Debug, Savefile)]
enum TodoItemState {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Savefile)]
struct TodoItem {
    state: TodoItemState,
    text: String,
}

impl TodoItem {
    fn new(text: &str) -> Self {
        Self {
            state: TodoItemState::Todo,
            text: text.to_string(),
        }
    }

    fn set_state(&mut self, state: TodoItemState) {
        self.state = state
    }

    fn to_string(&self) -> String {
        self.text.clone()
    }
}

#[derive(Debug, Savefile)]
struct TodoList {
    name: String,
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new(name: &str) -> TodoList {
        TodoList {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    fn add_item(&mut self, item: TodoItem) {
        self.items.push(item)
    }

    fn del_item(&mut self, index: usize) {
        self.items.remove(index);
    }

    fn del_item_fuzzy(&mut self, needle: String) {
        let matcher = SkimMatcherV2::default();
        for (index, item) in self.vec_string().iter().enumerate() {
            matcher.fuzzy_match(item, needle)
        }
    }

    fn vec_string(&mut self) -> Vec<String> {
        self.items.iter().map(|x| x.to_string()).collect()
    }

    fn format(&mut self) -> String {
        let mut res = String::new();
        for (index, item) in &mut self.vec_string().iter().enumerate() {
            res.push_str(format!("{}. {}\n", index, item).as_str())
        }
        res
    }
}

fn main() {
    let mut todo_list = TodoList::new("Some things that I need to do");
    todo_list.add_item(TodoItem::new("Take out the trash"));
    todo_list.add_item(TodoItem::new("Do homework"));
    println!("{}", todo_list.format());
}
