use fltk::{
    prelude::*,
    *,
};

mod greet;
use greet::Greet;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(600, 400).with_label("Greeting App");
    let mut greet = Greet::default();

    wind.end();
    wind.show();
    app.run().unwrap();
}
