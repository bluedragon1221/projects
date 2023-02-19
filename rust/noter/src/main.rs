use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

struct MyApp {
    counter: i32,
    button: button::State,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Message {
    ButtonPressed,
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> MyApp {
        MyApp {
            counter: 0,
            button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("My Iced App")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.counter += 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new("Welcome to my Iced app!"))
            .push(Text::new(format!("Counter: {}", self.counter)))
            .push(Button::new(&mut self.button, Text::new("Increment")))
            .into()
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}
