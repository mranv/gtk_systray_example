use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Menu, MenuItem};
use libappindicator::{AppIndicator, AppIndicatorStatus};

fn main() {
    let application = Application::new(
        Some("com.example.systray"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.set_title("Rust GTK Systray Example");
    window.set_default_size(350, 70);

    let mut indicator = AppIndicator::new("example-application", "dialog-information");
    indicator.set_status(AppIndicatorStatus::Active);

    let mut menu = Menu::new();
    
    let hello_item = MenuItem::with_label("Say Hello");
    hello_item.connect_activate(|_| {
        println!("Hello, World!");
    });
    menu.append(&hello_item);

    let quit_item = MenuItem::with_label("Quit");
    let app_clone = application.clone();
    quit_item.connect_activate(move |_| {
        app_clone.quit();
    });
    menu.append(&quit_item);

    menu.show_all();
    indicator.set_menu(&mut menu);

    window.show_all();
}