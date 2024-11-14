// FILE: main.rs

use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

mod app {
    pub mod ui {
        pub mod topbar;
        pub mod main {
            pub mod home;
        }
    }
    pub mod systems; // Add this line to include the systems module
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("au.yggdrasil.envmanager")
        .build();
    app.connect_activate(start);
    app.run();
}

pub fn start(application: &gtk::Application) {
    let window = Rc::new(RefCell::new(gtk::ApplicationWindow::new(application)));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let topbar = app::ui::topbar::create_topbar();
    let home = app::ui::main::home::create_home();

    vbox.append(&topbar);
    vbox.append(&home);

    // Ensure the topbar extends across the entire top
    topbar.set_hexpand(true);
    topbar.set_vexpand(false);

    // Ensure the main content starts beneath the topbar and fills the remaining space
    home.set_hexpand(true);
    home.set_vexpand(true);

    window.borrow().set_child(Some(&vbox));
    window.borrow().present();
}
