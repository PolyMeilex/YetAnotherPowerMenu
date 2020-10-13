use crate::config::Event;
use gtk::prelude::*;

pub struct State {
    last_key: Option<usize>,
}

impl State {
    pub fn new() -> Self {
        Self { last_key: None }
    }

    pub fn event(&mut self, ui: &crate::ui::GtkUi, e: Event) {
        ui.gtk_window.close();
        use Event::*;
        match e {
            Lock => {
                std::process::Command::new("i3lock")
                    .arg("-c")
                    .arg("000000")
                    // .arg("-i")
                    // .arg("/home/poly/Pictures/Wallpapers/11.png")
                    .spawn()
                    .ok();
            }
            Logout => {
                std::process::Command::new("i3-msg")
                    .arg("exit")
                    .spawn()
                    .ok();
            }
            Reboot => {
                std::process::Command::new("systemctl")
                    .arg("reboot")
                    .spawn()
                    .ok();
            }
            Shutdown => {
                std::process::Command::new("systemctl")
                    .arg("poweroff")
                    .spawn()
                    .ok();
            }
            Suspend => {
                std::process::Command::new("systemctl")
                    .arg("suspend")
                    .spawn()
                    .ok();
            }
            Custom(c) => {
                let mut command = c.into_iter();
                let program = command.next();

                if let Some(program) = program {
                    let args: Vec<String> = command.collect();
                    std::process::Command::new(&program).args(args).spawn().ok();
                }
            }
        }
    }

    pub fn key_event(&mut self, ui: &crate::ui::GtkUi, id: usize, e: Event) {
        if let Some(last_key) = self.last_key {
            if last_key == id {
                self.event(ui, e);
            }
        }

        self.last_key = Some(id);
    }
}
