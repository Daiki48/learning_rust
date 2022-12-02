#![windows_subsystem = "windows"]

use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300).with_label("Dialog - example");

    let mut btn = button::Button::default()
        .with_size(80, 30)
        .with_label("Select Files")
        // .with_label("ファイルを選択")
        .center_of_parent();

    wind.end();
    wind.show();

    btn.set_callback(|_| {
        let mut dialog = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseFile);
        dialog.show();
        // ファイルパスをコンソール画面に出力
        println!("{:?}", dialog.filename());
    });

    app.run().unwrap();
}
