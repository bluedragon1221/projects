use std::fmt::{Display, Write};

use list_comprehension_macro::comp;

use inquire;

#[derive(Debug, thiserror::Error)]
pub enum TodoError {
    #[error("There was an error when formatting a string")]
    FormatError(#[from] std::fmt::Error),

    #[error("Selector error")]
    SelectError(#[from] inquire::InquireError),

    #[error("Item not in list")]
    NoItem,
}

#[derive(Clone, Eq, PartialEq)]
pub enum TodoItemState {
    Todo,
    InProgress,
    Done,
}

impl TodoItemState {
    fn format(&self) -> String {
        format!(
            "[{}]",
            match self {
                Self::Todo => String::from(' '),
                Self::InProgress => String::from('-'),
                Self::Done => String::from('X'),
            }
        )
    }

    fn toggle(&mut self) {
        *self = match &self {
            Self::Todo => Self::InProgress,
            Self::InProgress => Self::Done,
            Self::Done => Self::Todo,
        }
    }

    fn set_state(&mut self, new_state: Self) {
        *self = new_state
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TodoItem {
    state: TodoItemState,
    text: String,
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            state: TodoItemState::Todo,
            text: text.to_string(),
        }
    }

    pub fn get_state(&self) -> TodoItemState {
        self.state.clone()
    }

    pub fn set_state(&mut self, new_state: TodoItemState) {
        self.state.set_state(new_state)
    }

    pub fn toggle(&mut self) {
        self.state.toggle()
    }

    fn format(&self) -> String {
        format!("{} {}", self.state.format(), self.text.clone())
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.state.format(), self.text.clone())
    }
}

pub struct TodoList {
    name: String,
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new(name: &str) -> TodoList {
        TodoList {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: TodoItem) {
        self.items.push(item)
    }

    pub fn del_item(&mut self, index: usize) {
        self.items.remove(index);
    }

    pub fn mut_by_index(&mut self, index: usize) -> &mut TodoItem {
        &mut self.items[index]
    }

    pub fn get_mut_state(&mut self, state: TodoItemState) -> Vec<&mut TodoItem> {
        comp![i for i in &mut self.items if i.get_state() == state]
    }

    pub fn get_items(&self) -> Vec<TodoItem> {
        self.items.clone()
    }

    pub fn format(&mut self) -> Result<String, TodoError> {
        self.items
            .iter()
            .enumerate()
            .try_fold(String::new(), |mut acc, (index, item)| {
                writeln!(acc, "{}. {}", index + 1, item.format())?;
                Ok(acc)
            })
    }

    pub fn get_index_of(&self, item: TodoItem) -> Result<usize, TodoError> {
        self.items
            .iter()
            .position(|r| *r == item)
            .ok_or(TodoError::NoItem)
    }
}
