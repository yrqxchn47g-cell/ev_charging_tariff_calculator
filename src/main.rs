use iced::{Application, Command, Element, Settings, Text};

pub fn main() -> iced::Result {
    BreakevenApp::run(Settings::default())
}

struct BreakevenApp;

impl Application for BreakevenApp {
    type Executor = iced::executor::Default;
    type Message = ();  // Modify this to define messages as needed
    type Flags = ();    // Modify this to handle application flags if needed

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (BreakevenApp, Command::none())
    }

    fn title(&self) -> String {
        String::from("EV Charging Tariff Breakeven Analysis")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Welcome to the EV Charging Tariff Breakeven Analysis Application!").into()
    }
}