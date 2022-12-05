use fltk::{
    app,
    enums,
    prelude::{
        GroupExt,
        WidgetExt,
    },
    window,
};

use fltk_table::{
    SmartTable,
    TableOpts,
};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(800, 600);

    let mut table = SmartTable::default()
        .with_size(790, 590)
        .center_of_parent()
        .with_opts(TableOpts {
            rows: 30,
            cols: 15,
            editable: true,   // セルの値を変更できるようにしている。
            ..Default::default()
        });

    wind.end();

    // ウィンドウを可変に
    wind.make_resizable(true);

    wind.show();

    // 行と列の値を足してセルにその値を入れていってる。
    for i in 0..30 {
        for j in 0..15 {
            table.set_cell_value(i, j, &(i + j).to_string());
        }
    }

    // 0スタートなので、これは、
    // 4行目の5列目にanotherという値を直接入れてる。
    table.set_cell_value(3, 4, "another");

    assert_eq!(table.cell_value(3, 4), "another");

    // エスケープでアプリが落ちないように
    wind.set_callback(move |_| {
        if app::event() == enums::Event::Close {
            app.quit();
        }
    });

    app.run().unwrap();
    
}
