use crate::config::Config;
use gdk::prelude::*;
use glib::object::IsA;
use gtk::prelude::*;

pub struct GtkUi {
    /// Main Window
    pub gtk_window: gtk::Window,
    /// Window Geometry
    /// (x,y,w,h)
    pub window_geometry: (i32, i32, i32, i32),
    /// Date and time widget
    pub date_time: DateTimeWidget,
    /// Buttons group widget
    pub button_group: ButtonGroupWidget,
}

impl GtkUi {
    pub fn new(config: &Config) -> Self {
        let gtk_window = {
            let gtk_window = gtk::WindowBuilder::new()
                .name("root-window")
                .type_(gtk::WindowType::Popup)
                .window_position(gtk::WindowPosition::Center)
                .build();

            let screen = gtk_window.get_screen().unwrap();
            let monitor = screen.get_primary_monitor();

            let gdk::Rectangle {
                x: _,
                y: _,
                width,
                height,
            } = screen.get_monitor_geometry(monitor);

            gtk_window.set_property_width_request(width);
            gtk_window.set_property_height_request(height);
            gtk_window.set_skip_pager_hint(true);
            gtk_window.set_keep_above(true);

            set_visual(&gtk_window, None);

            gtk_window
        };

        let root_box = gtk::BoxBuilder::new()
            .parent(&gtk_window)
            .name("root-box")
            .orientation(gtk::Orientation::Vertical)
            .margin(20)
            .spacing(20)
            .valign(gtk::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build();

        let date_time = DateTimeWidget::new(config).set_parent(&root_box);
        let button_group = ButtonGroupWidget::new(config).set_parent(&root_box);

        gtk_window.show_all();

        // Some aditional setup after window is created (shown)
        let window = gtk_window.get_window().unwrap();
        let window_geometry = window.get_geometry();

        // Grab a seat
        {
            let display = window.get_display();
            let seat = display.get_default_seat().unwrap();

            // Atempt to grab a seat
            let mut atempts = 0;
            loop {
                let status = seat.grab(&window, gdk::SeatCapabilities::ALL, true, None, None, None);

                match status {
                    gdk::GrabStatus::Success => {
                        // No need to retry anymore we can just break
                        break;
                    }
                    _ => {
                        eprintln!("{:?}", status);
                    }
                }

                atempts += 1;

                // Let's wait 100ms in hope that grab will be posible later
                std::thread::sleep(std::time::Duration::from_millis(100));

                // Let make shure that we don't block the user view if something goes wrong, after 1s of attempts we should give up and go away.
                // Otherwise, the user will have to deal with an unresponsive window that obscures the whole view himself, we definitely don't want that to happen XD
                if atempts > 10 {
                    panic!("Grab attempts exceeded 10 (1s)");
                }
            }
        }

        Self {
            gtk_window,
            window_geometry,
            date_time,
            button_group,
        }
    }
}

/// Date and time widget
pub struct DateTimeWidget {
    pub root: gtk::Box,
    pub time_label: gtk::Label,
    pub date_label: gtk::Label,
}

impl DateTimeWidget {
    pub fn new(config: &Config) -> Self {
        let root = gtk::BoxBuilder::new()
            .name("date-time-root")
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .build();

        let time = chrono::Local::now();

        let time_label = gtk::LabelBuilder::new()
            .label(&time.format(&config.time_format).to_string())
            .name("time")
            .parent(&root)
            .build();

        let date_label = gtk::LabelBuilder::new()
            .label(&time.format(&config.date_format).to_string())
            .name("date")
            .parent(&root)
            .build();

        Self {
            root,
            time_label,
            date_label,
        }
    }
    pub fn set_parent<P: IsA<gtk::Container>>(self, widget: &P) -> Self {
        widget.add(&self.root);
        self
    }
}

/// Buttons group widget
pub struct ButtonGroupWidget {
    pub root: gtk::Box,
    pub buttons: Vec<gtk::Button>,
}

impl ButtonGroupWidget {
    pub fn new(config: &Config) -> Self {
        let root = gtk::BoxBuilder::new()
            .name("btn-group-root")
            .spacing(10)
            .halign(gtk::Align::Center)
            .build();

        let buttons = config
            .buttons
            .iter()
            .map(|btn| {
                let btn = gtk::Button::from_icon_name(Some(&btn.icon), gtk::IconSize::Button);
                root.add(&btn);
                btn.get_style_context().add_class("btn");

                btn
            })
            .collect();

        Self { root, buttons }
    }
    pub fn set_parent<P: IsA<gtk::Container>>(self, widget: &P) -> Self {
        widget.add(&self.root);
        self
    }
}

/// Set visual for window (used for transparency)
fn set_visual(window: &gtk::Window, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(ref visual) = screen.get_rgba_visual() {
            window.set_visual(Some(visual));
        }
    }
}
