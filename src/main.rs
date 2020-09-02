use gio::prelude::*;

mod app;
mod state;
mod ui;

fn main() {
    let app = gtk::ApplicationBuilder::new()
        .application_id("io.github.polymeilex.yal")
        .build();

    app.connect_activate(|app| {
        let app = app::App::new(app);
        app.connect();
    });

    app.run(&std::env::args().collect::<Vec<_>>());
}
