use glib::clone;

use gtk::glib;
use gtk::prelude::*;

fn on_activate(application: &gtk::Application) {
    // … create a new window …
    let window = gtk::ApplicationWindow::new(application);
    // … with a button in it …
    let button = gtk::Button::with_label("Hello World!");
    // … which closes the window when clicked
    button.connect_clicked(clone!(@weak window => move |_| window.close()));
    window.set_child(Some(&button));
    window.present();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    app.run();
}