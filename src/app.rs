use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::{config::Config, state::State, ui::GtkUi};

pub struct App {
    ui: Rc<GtkUi>,
    state: Rc<RefCell<State>>,
    config: Rc<Config>,
}

impl App {
    pub fn new(app: &gtk::Application, config: Config) -> Self {
        // Stylesheet
        {
            // let file = std::fs::read("/home/poly/Documents/rust/YetAnotherPowerMenu/src/style.css")
            //     .unwrap();

            let file = include_bytes!("style.css");

            let style_provider = gtk::CssProvider::new();
            style_provider.load_from_data(file).unwrap();
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().unwrap(),
                &style_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        Self {
            ui: Rc::new(GtkUi::new(app, &config)),
            state: Rc::new(RefCell::new(State::new())),
            config: Rc::new(config),
        }
    }

    pub fn connect(&self) {
        // Date and time timer
        {
            let ui = self.ui.clone();
            let config = self.config.clone();

            glib::timeout_add_local(1000, move || {
                let now = chrono::Local::now();

                let time = now.format(&config.time_format).to_string();
                let date = now.format(&config.date_format).to_string();

                ui.date_time.time_label.set_text(&time);
                ui.date_time.date_label.set_text(&date);

                glib::Continue(true)
            });
        }

        // Key events
        {
            let ui = self.ui.clone();
            let state = self.state.clone();
            let config = self.config.clone();

            self.ui.gtk_window.connect_key_press_event(move |_, ev| {
                let ev = ev.get_keyval();

                let mut state = state.borrow_mut();
                let win = &ui.gtk_window;

                for btn in config.buttons.iter() {
                    match &ev {
                        ev if { ev == &*btn.key } => state.event(&ui, btn.event.clone()),
                        &gdk::keys::constants::Escape => win.close(),
                        _ => {}
                    }
                }

                Inhibit(true)
            });
        }

        // Btn events
        for (btn, gtk_btn) in self
            .config
            .buttons
            .iter()
            .zip(self.ui.button_group.buttons.iter())
        {
            let state = self.state.clone();
            let ui = self.ui.clone();
            let event = btn.event.clone();

            gtk_btn.connect_clicked(move |_gtk_btn| {
                let mut state = state.borrow_mut();
                state.event(&ui, event.clone());
            });
        }
    }
}
