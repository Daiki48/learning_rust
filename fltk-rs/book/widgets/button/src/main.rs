use fltk::{
    prelude::*,
    *
};

fn main() {
    // endとshowの前にbuttonを作る。
    // let app = app::App::default();
    // let mut wind = window::Window::new(100, 100, 400, 300, "My Window");
    // let _btn = button::Button::new(160, 200, 80, 40, "First button");
    // wind.end();
    // wind.show();
    // app.run().unwrap();

    // endとshowの後に、buttonを追加（add）する。
    let app = app::App::default();
    let mut wind = window::Window::new(100, 100, 400, 300, "My Window");

    wind.end();
    wind.show();

    let btn = button::Button::new(160, 200, 80, 40, "First button");
    wind.add(&btn);

    app.run().unwrap();
}
