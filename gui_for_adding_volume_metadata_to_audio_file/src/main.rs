use iced::widget::{column, container, row};
use iced::{Element, Fill};

pub fn main() -> iced::Result {
    iced::run(update, view)
}

fn view(counter: &Counter) -> Element<'_, Message> {
    container(column!["Top", row!["Left", "Right"].spacing(10), "Bottom"].spacing(10))
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
}
fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u64,
}
//test
