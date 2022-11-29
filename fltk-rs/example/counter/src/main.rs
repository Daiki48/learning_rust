use fltk::{
    app,
    button::Button,
    enums::{Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(0x62, 0x00, 0xee);
    app::set_visible_focus(false);

    // windは、window自身のサイズや上のバーにラベルを設定する。
    let mut wind = Window::default().with_size(400, 400).with_label("Counter App");

    // frameは、表示される数字などのフレーム
    // おそらく、windowのなかに描画用のフレームがあり、それを指している？
    let mut frame = Frame::default()
        .with_size(100, 40)
        .center_of(&wind)
        .with_label("0");

    // frameに表示されるlabelのフォントサイズ
    // 今回だと、0という数字の大きさ
    frame.set_label_size(40); // あんまり大きくすると、上下のpaddingにぶつかって文字が崩れる。

    let mut btn_inc = Button::default()
        .size_of(&frame)
        .above_of(&frame, 40) // 第二引数はpaddingの設定値
        .with_label("+");
    btn_inc.set_label_size(50); // +のフォントサイズを調整

    let mut btn_dec = Button::default()
        .size_of(&frame)
        .below_of(&frame, 40)
        .with_label("-");
    btn_dec.set_label_size(50);

    wind.end();
    wind.show();

    // +ボタンを押すと、真ん中の数字がインクリメントされる。
    btn_inc.set_callback({
        let mut frame = frame.clone();
        move |_| {
            let label = (frame.label().parse::<i32>().unwrap() + 1).to_string();
            frame.set_label(&label);
        }
    });

    // -ボタンを押すと、真ん中の数字がデクリメントされる。
    btn_dec.set_callback({
        // let mut frame = frame.clone();
        move |_| {
            let label = (frame.label().parse::<i32>().unwrap() - 1).to_string();
            frame.set_label(&label);
        }
    });

    // Theming
    // ボタンと真ん中の数字以外の背景を白色にする。
    wind.set_color(Color::White);

    // 少し明るい青色にする。
    btn_inc.set_color(Color::from_u32(0x304FFE));

    // ボタンを押したときの色を緑色にする。
    btn_inc.set_selection_color(Color::Green);

    // ボタンの縁を無くして、3Dボタンから2ボタンへと変更
    btn_inc.set_frame(FrameType::FlatBox);

    // +のフォントカラーを白色にする。
    btn_inc.set_label_color(Color::White);

    // ここから下はデクリメントボタンのスタイリング
    // 内容は上で説明している項目と同じ
    btn_dec.set_color(Color::from_u32(0x2962FF));
    btn_dec.set_selection_color(Color::Red);
    btn_dec.set_frame(FrameType::FlatBox);
    btn_dec.set_label_color(Color::White);

    app.run().unwrap();
}
