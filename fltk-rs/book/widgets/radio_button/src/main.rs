use fltk::{
    prelude::*,
    *,
};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);

    // このwith_sizeは、ラジオボタンそれぞれの選択出来る範囲。
    // 点線で範囲が見えるようになっているはず。
    let flex = group::Flex::default().with_size(200, 100).column().center_of_parent();

    let _btn1 = button::RadioRoundButton::default().with_label("Radio Button 1");
    let _btn2 = button::RadioRoundButton::default().with_label("Radio Button 2");
    flex.end();
    wind.end();
    wind.show();
    app.run().unwrap();
}
