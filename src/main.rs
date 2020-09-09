mod app;
mod state;
mod ui;

mod config;

fn main() {
    gtk::init().unwrap();

    let (config, style) = config::get_config();
    // Stylesheet
    {
        use gtk::prelude::*;

        let style_provider = gtk::CssProvider::new();
        style_provider.load_from_data(&style).unwrap();
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().unwrap(),
            &style_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let app = app::App::new(config);
    app.connect();

    gtk::main();
}

pub fn quick_dialog(msg: &str) {
    use gtk::prelude::*;

    let d = gtk::MessageDialogBuilder::new()
        .message_type(gtk::MessageType::Error)
        .buttons(gtk::ButtonsType::Ok)
        .text(msg)
        .build();

    d.run();
    unsafe { d.destroy() };
}
