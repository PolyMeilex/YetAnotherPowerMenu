use gdk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;
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
            let file = std::fs::read(
                "/home/poly/Documents/Programing/rust/YetAnotherPowerMenu/src/style.css",
            )
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
        let ui = self.ui.clone();
        let state = self.state.clone();

        self.ui.gtk_window.connect_key_press_event(move |_, ev| {
            let ev = ev.get_keyval();

            // let controler = stae.borrow();
            let win = &ui.gtk_window;

            match ev {
                gdk::keys::constants::Escape => win.close(),
                gdk::keys::constants::j => win.close(),
                gdk::keys::constants::k => win.close(),
                gdk::keys::constants::l => win.close(),
                gdk::keys::constants::semicolon => win.close(),
                _ => {}
            }

            gtk::Inhibit(false)
        });

        // let ui = self.ui.clone();
        // self.ui.gtk_window.connect_button_press_event(move |_, ev| {
        //     let (x, y) = ev.get_position();

        //     let (_wx, _wy, ww, wh) = ui.window_geometry;
        //     if !(x > 0.0 && x < ww as f64 && y > 0.0 && y < wh as f64) {
        //         ui.gtk_window.close();
        //     }

        //     gtk::Inhibit(false)
        // });

        let ui = self.ui.clone();
        self.ui.gtk_window.connect_leave_notify_event(move |_, _| {
            ui.gtk_window.close();
            Inhibit(false)
        });
    }
}
