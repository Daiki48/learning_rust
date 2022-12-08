use fltk::{
    prelude::*,
    *,
};
use fltk_grid::Grid;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(600, 400).with_label("Greeting App");
    let mut grid = Grid::default_fill();
    grid.debug(true);
    grid.set_layout(8, 6);

    let mut title = frame::Frame::default().with_label("Greeting!");
    title.set_frame(enums::FrameType::FlatBox);
    title.set_color(enums::Color::DarkBlue);
    title.set_label_color(enums::Color::White);
    title.set_label_size(40);
    grid.insert(&mut title, 0..4, 1);

    grid.insert(&mut button::Button::default(), 2..6, 1..4);

    wind.end();
    wind.show();
    app.run().unwrap();
}
