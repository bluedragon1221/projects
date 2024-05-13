use cursive::view::{Resizable, Scrollable};
use cursive::views::{Dialog, SelectView};
use todo::{TodoError, TodoItem, TodoList};

fn main() -> Result<(), TodoError> {
    let mut todo_list = TodoList::new("Some things that I need to do");

    todo_list.add_item(TodoItem::new("Take out the trash"));
    todo_list.add_item(TodoItem::new("Do homework"));

    let mut select = SelectView::new().autojump();

    select.add_all_str(todo_list.format()?.split('\n'));

    let mut siv = cursive::default();

    siv.add_layer(Dialog::around(select.scrollable().fixed_size((30, 20))).title("Select"));

    siv.run();

    Ok(())
}
