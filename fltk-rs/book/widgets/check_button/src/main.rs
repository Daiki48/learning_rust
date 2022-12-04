use fltk::{
    prelude::*,
    *,
};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300).with_label("Check Button");
    let flex = group::Flex::default().with_size(200, 100).column().center_of_parent();
    let mut btn1 = button::CheckButton::default().with_label("check button1");
    // 下記のように、set_valueで初期値を設定できる。
    // set_value(true)にすると、アプリを起動した瞬間からチェックが入っている。
    btn1.set_value(true);
    let btn2 = button::CheckButton::default().with_label("check button2");
    let mut btn3 = button::Button::default().with_label("submit");
    flex.end();
    wind.end();
    wind.show();

    btn3.set_callback(move |_btn3| {
        if btn1.value() {
            println!("btn1 is checked");
        }
        if btn2.value() {
            println!("btn2 is checked");
        }
    });

    app.run().unwrap();
}
