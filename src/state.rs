use crate::config::Event;
use gtk::prelude::*;

pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn event(&mut self, ui: &crate::ui::GtkUi, e: Event) {
        use Event::*;
        match e {
            Lock => {
                ui.gtk_window.close();

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
                std::process::Command::new("reboot").spawn().ok();
            }
            Shutdown => {
                std::process::Command::new("poweroff").spawn().ok();
            }
            Suspend => {
                std::process::Command::new("suspend").spawn().ok();
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
}
