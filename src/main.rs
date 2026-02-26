use iced::widget::Text;
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    BreakevenApp::run(Settings::default())
}

#[derive(Default)]
struct BreakevenApp;

impl Sandbox for BreakevenApp {
    type Message = ();

    fn new() -> Self {
        BreakevenApp
    }

    fn title(&self) -> String {
        String::from("EV Charging Tariff Breakeven Analysis")
    }

    fn update(&mut self, _message: Self::Message) {
    }

    fn view(&self) -> Element<Self::Message> {
        Text::new("Welcome to the EV Charging Tariff Breakeven Analysis Application!")
            .into()
    }
}