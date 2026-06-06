use iced::Element;
use iced::widget::{button, text};
pub fn main() -> iced::Result {
    iced::run(update, view)
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

fn view(counter: &Counter) -> Element<'_, Message> {
    button(text(counter.value))
        .on_press(Message::Increment)
        .into()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u64,
}
