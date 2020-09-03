use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::{state::State, ui::GtkUi};

pub struct App {
    ui: Rc<GtkUi>,
    state: Rc<RefCell<State>>,
}

impl App {
    pub fn new(app: &gtk::Application) -> Self {
        // Stylesheet
        {
            let file = std::fs::read("/home/poly/Documents/rust/YetAnotherPowerMenu/src/style.css")
                .unwrap();

            let style_provider = gtk::CssProvider::new();
            style_provider.load_from_data(&file).unwrap();
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().unwrap(),
                &style_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        Self {
            ui: Rc::new(GtkUi::new(app)),
            state: Rc::new(RefCell::new(State::new())),
        }
    }

    pub fn connect(&self) {
        use crate::state::Event;

        let ui = self.ui.clone();
        let state = self.state.clone();

        self.ui.gtk_window.connect_key_press_event(move |_, ev| {
            let ev = ev.get_keyval();

            let mut state = state.borrow_mut();
            let win = &ui.gtk_window;

            match ev {
                gdk::keys::constants::Escape => win.close(),
                gdk::keys::constants::j => state.event(Event::Reboot),
                gdk::keys::constants::k => {
                    win.close();
                    state.event(Event::Lock);
                }
                gdk::keys::constants::l => state.event(Event::Logout),
                gdk::keys::constants::semicolon => state.event(Event::Shutdown),
                _ => {}
            }

            gtk::Inhibit(false)
        });

        let state = self.state.clone();
        self.ui.button_group.buttons.0.connect_clicked(move |btn| {
            let mut state = state.borrow_mut();
            state.event(Event::Reboot);
        });

        let ui = self.ui.clone();
        let state = self.state.clone();
        self.ui.button_group.buttons.1.connect_clicked(move |btn| {
            let mut state = state.borrow_mut();
            let win = &ui.gtk_window;

            win.close();
            state.event(Event::Lock);
        });

        let state = self.state.clone();
        self.ui.button_group.buttons.2.connect_clicked(move |btn| {
            let mut state = state.borrow_mut();
            state.event(Event::Logout);
        });

        let state = self.state.clone();
        self.ui.button_group.buttons.3.connect_clicked(move |btn| {
            let mut state = state.borrow_mut();
            state.event(Event::Shutdown);
        });
    }
}
