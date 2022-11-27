// #![windows_subsystem = "windows"]

use winsafe::{
    prelude::*,
    gui,
    POINT,
    SIZE,
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct MyWindow {
    pub wnd: gui::WindowMain,
    pub lbl_title: gui::Label,
    pub description: gui::Label,
    pub select_number: gui::ComboBox,
}

impl MyWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "Guessing Game".to_owned(),
                size: SIZE { cx: 300, cy: 300 },
                ..Default::default()
            },
        );

        let lbl_title = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Guessing Game".to_owned(),
                position: POINT { x: 100, y: 10 },
                // 下記の設定を諸々全て..Default::default()で設定している
                // size: SIZE { cx: 100, cy: 50 },
                // label_style: (),
                // window_style: (),
                // window_ex_style: (),
                // ctrl_id: (),
                // horz_resize: (),
                // vert_resize: ()
                ..Default::default()
            },
        );

        let description = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "The game is to guess the hidden numbers.".to_owned(),
                position: POINT { x: 30, y: 40 },
                ..Default::default()
            },
        );

        let select_number = gui::ComboBox::new(
            &wnd,
            gui::ComboBoxOpts {
                position: POINT::new(80, 100),
                width: 140,
                // combo_box_style: (),
                // window_style: (),
                // window_ex_style: (),
                // ctrl_id: (),
                // horz_resize: (),
                // vert_resize: (),
                // items: (),
                // selected_item: (),
                ..Default::default()
            },
        );

        let new_self = Self {wnd, lbl_title, description, select_number};
        new_self.events();
        new_self
    }

    fn events(&self) {
        self.wnd.on().wm_create({
            let self2 = self.clone();
            move |_| {
                // self2.select_number.items().add(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
                self2.select_number.items().add(&["Hello", "World"]);
                Ok(0)
            }
        });

        self.select_number.on().cbn_sel_change({
            let self2 = self.clone();
            move || {
                if let Some(num) = self2.select_number.items().selected_text() {
                    self2.wnd.hwnd().SetWindowText(&num)?;
                }
                Ok(())
            }
        });
    }
}
