use gtk::prelude::*;
use gtk::{Box, Label};
use crate::app::systems::util::get_env_vars;

pub fn create_home() -> Box {
    let home = Box::new(gtk::Orientation::Vertical, 0);
    // Add widgets to the home as needed

    let env_vars = get_env_vars();
    for (key, value) in env_vars.iter() {
        let label = Label::new(Some(&format!("{}: {}", key, value)));
        home.append(&label);
    }

    home
}