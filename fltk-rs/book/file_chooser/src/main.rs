use fltk::{
    prelude::*,
    *,
};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);

    let mut btn = button::Button::default()
        .with_size(100, 30)
        .with_label("Select File")
        .center_of_parent();

    wind.end();
    wind.show();

    btn.set_callback(|_| {
        let mut dialog = dialog::FileChooser::new(
            ".",
            "*.{txt,rs,toml}",
            dialog::FileChooserType::Multi,
            "Select file",
        );
        dialog.show();

        while dialog.shown() {
            app::wait();
        }
        if dialog.count() > 1 {
            for i in 1..=dialog.count() {
                println!("VALUE[{}]: '{}'", i, dialog.value(i).unwrap());
            }
        }
    });

    app.run().unwrap();

}
