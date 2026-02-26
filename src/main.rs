use iced::widget::text;
use iced::Element;

pub fn main() -> iced::Result {
    iced::run(
        "EV Charging Tariff Breakeven Analysis",
        BreakevenApp::update,
        BreakevenApp::view,
    )
}

#[derive(Default)]
struct BreakevenApp;

#[derive(Debug, Clone)]
enum Message {}

impl BreakevenApp {
    fn update(&mut self, _message: Message) -> iced::Task<Message> {
        iced::Task::none()
    }

    fn view(&self) -> Element<Message> {
        text("Welcome to the EV Charging Tariff Breakeven Analysis Application!").into()
    }
}