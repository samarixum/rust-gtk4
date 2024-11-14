use gtk::prelude::*;
use gtk::{Box, Button, Orientation};

pub fn create_topbar() -> Box {
    let topbar = Box::new(Orientation::Horizontal, 0);

    let home_button = Button::with_label("Home");
    let settings_button = Button::with_label("Settings");

    topbar.append(&home_button);
    topbar.append(&settings_button);

    // Add more widgets to the topbar as needed
    topbar
}