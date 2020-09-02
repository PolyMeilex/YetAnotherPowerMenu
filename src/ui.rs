use gdk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;
use gtk::prelude::*;

pub struct GtkUi {
    pub gtk_window: gtk::ApplicationWindow,
    pub window_geometry: (i32, i32, i32, i32),
    pub date_time: DateTimeWidget,
    pub button_group: ButtonGroupWidget,
}

impl GtkUi {
    pub fn new(app: &gtk::Application) -> Self {
        let gtk_window = gtk::ApplicationWindowBuilder::new()
            .application(app)
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

        let root_box = gtk::BoxBuilder::new()
            .parent(&gtk_window)
            .name("root-box")
            .orientation(gtk::Orientation::Vertical)
            .margin(20)
            .spacing(20)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        let date_time = DateTimeWidget::new();
        root_box.add(&date_time.root);

        let button_group = ButtonGroupWidget::new();
        root_box.add(&button_group.root);

        gtk_window.show_all();

        let window_geometry = {
            let window = gtk_window.get_window().unwrap();
            let display = window.get_display();
            let seat = display.get_default_seat().unwrap();

            seat.grab(&window, gdk::SeatCapabilities::ALL, true, None, None, None);

            window.get_geometry()
        };

        Self {
            gtk_window,
            window_geometry,
            date_time,
            button_group,
        }
    }
}

pub struct DateTimeWidget {
    pub root: gtk::Box,
    pub time_label: gtk::Label,
    pub date_label: gtk::Label,
}

impl DateTimeWidget {
    pub fn new() -> Self {
        let root = gtk::BoxBuilder::new()
            .name("date-time-root")
            .orientation(gtk::Orientation::Vertical)
            .build();

        let time = chrono::Local::now();
        let time_label = gtk::LabelBuilder::new()
            .label(&time.format("%H:%M").to_string())
            .name("time")
            .parent(&root)
            .build();

        let date_label = gtk::LabelBuilder::new()
            .label(&time.format("%A, %d %B %Y").to_string())
            .name("date")
            .parent(&root)
            .build();

        Self {
            root,
            time_label,
            date_label,
        }
    }
}

pub struct ButtonGroupWidget {
    pub root: gtk::Box,
    pub buttons: (gtk::Button, gtk::Button, gtk::Button, gtk::Button),
}

impl ButtonGroupWidget {
    pub fn new() -> Self {
        let root = gtk::BoxBuilder::new()
            .name("btn-group-root")
            .spacing(10)
            .halign(gtk::Align::Center)
            .build();

        // let btn0 = gtk::ButtonBuilder::new().label("j").parent(&root).build();
        let btn0 =
            gtk::Button::from_icon_name(Some("system-reboot-symbolic"), gtk::IconSize::Button);
        root.add(&btn0);
        btn0.get_style_context().add_class("btn");

        let btn1 =
            gtk::Button::from_icon_name(Some("system-lock-screen-symbolic"), gtk::IconSize::Button);
        root.add(&btn1);
        btn1.get_style_context().add_class("btn");

        let btn2 =
            gtk::Button::from_icon_name(Some("system-log-out-symbolic"), gtk::IconSize::Button);
        root.add(&btn2);
        btn2.get_style_context().add_class("btn");

        let btn3 =
            gtk::Button::from_icon_name(Some("system-shutdown-symbolic"), gtk::IconSize::Button);
        root.add(&btn3);
        btn3.get_style_context().add_class("btn");

        Self {
            root,
            buttons: (btn0, btn1, btn2, btn3),
        }
    }
}

fn set_visual(window: &gtk::ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(ref visual) = screen.get_rgba_visual() {
            window.set_visual(Some(visual));
        }
    }
}
