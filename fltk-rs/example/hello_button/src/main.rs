#![windows_subsystem = "windows"]

use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::*,
    window::Window,
};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300).with_label("Hello Button!");
    let mut frame = Frame::default().with_size(200, 100).center_of(&wind);
    let mut hello_btn = Button::new(160, 210, 80, 40, "Click me!");
    let mut clear_btn = Button::new(160, 40, 80, 40, "Clear");

    wind.end();
    wind.show();

    hello_btn.set_callback({
        let mut frame = frame.clone();
        move |_| frame.set_label("Hello world")
    });

    // clearボタンを新しく作って、
    // それを押すとframeに空白を設定するようにした。
    // 実質クリア出来てるわけではない・・・
    clear_btn.set_callback({
        move |_| frame.set_label("")
    });

    app.run().unwrap();
}
