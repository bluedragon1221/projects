use iced::widget::{button, column, text, Column};
use iced::Alignment;

fn main() -> iced::Result {
    iced::program("counter", Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_items(Alignment::Center)
    }
}
