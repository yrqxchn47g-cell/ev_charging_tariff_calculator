// src/ui.rs

use gtk::prelude::*; // GTK traits
use gtk::{Button, Label, Tooltip};
use chart::{Chart, ChartType}; // Hypothetical chart library

fn main() {
    // Initialize GTK application
    let application = gtk::Application::new(Some("com.example.ev_charging_tariff_calculator"), Default::default());

    application.connect_activate(|app| {
        // Create main window
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("EV Charging Tariff Calculator");
        window.set_default_size(350, 70);

        // Create a vertical box layout
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

        // Create a label
        let label = Label::new(Some("Welcome to the EV Charging Tariff Calculator!"));
        vbox.pack_start(&label, true, true, 0);

        // Create a button with tooltip
        let button = Button::with_label("Calculate Tariff");
        button.set_tooltip_text(Some("Click to calculate your EV charging tariff"));
        button.connect_clicked(|_| {
            // Handle button click
            println!("Calculating tariff...");
        });
        vbox.pack_start(&button, true, true, 0);

        // Create a chart visualization
        let chart = Chart::new(ChartType::Bar);
        vbox.pack_start(&chart, true, true, 0);

        // Add the VBox to the window and display
        window.add(&vbox);
        window.show_all();
    });

    application.run();
}