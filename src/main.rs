use gio::prelude::*;

mod app;
mod state;
mod ui;

mod config;

fn main() {
    let app = gtk::ApplicationBuilder::new().application_id("io.github.polymeilex.yal").build();

    app.connect_activate(|app| {
        let config = config::Config::deserialize();
        let app = app::App::new(app, config);
        app.connect();
    });

    app.run(&std::env::args().collect::<Vec<_>>());
}
