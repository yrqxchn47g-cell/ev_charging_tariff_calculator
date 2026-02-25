use iced::{Application, Command, Element, Settings};
use iced::widget::{button, column, text};

pub fn main() -> iced::Result {
    BreakevenApp::run(Settings::default())
}

struct BreakevenApp;

#[derive(Debug, Clone)]
enum Message {
    OpenHelp,
}

impl Application for BreakevenApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (BreakevenApp, Command::none())
    }

    fn title(&self) -> String {
        String::from("EV Charging Tariff Breakeven Analysis")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::OpenHelp => {
                let help_path = std::env::current_exe()
                    .ok()
                    .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                    .map(|p| p.join("docs").join("hilfe.pdf"))
                    .unwrap_or_else(|| std::path::PathBuf::from("docs/hilfe.pdf"));

                let path = if help_path.exists() {
                    help_path
                } else {
                    std::path::PathBuf::from("docs/hilfe.pdf")
                };

                if let Err(e) = opener::open(&path) {
                    eprintln!("Fehler beim Oeffnen der Hilfedatei: {}", e);
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let welcome = text("Willkommen zum EV Charging Tariff Breakeven Analysis!")
            .size(20);

        let help_button = button("? Hilfe")
            .on_press(Message::OpenHelp)
            .padding(10);

        column![welcome, help_button]
            .spacing(20)
            .padding(30)
            .into()
    }
}