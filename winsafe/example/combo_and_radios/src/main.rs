#![windows_subsystem = "windows"]

use winsafe::{
    prelude::*,
    gui,
    POINT,
    SIZE,
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct MyWindow {
    wnd: gui::WindowMain,
    lbl_title: gui::Label,
    cmb_lang: gui::ComboBox,
    rad_editor: gui::RadioGroup,
}

impl MyWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "Combo and Radios".to_owned(),
                // class_icon: gui::Icon::Id(101),    // error [exit code 101]
                size: SIZE { cx: 300, cy: 300 },
                ..Default::default()
            },
        );

        let lbl_title = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "This is Title".to_owned(),
                position: POINT { x: 20, y: 10 },
                ..Default::default()
            },
        );

        let cmb_lang = gui::ComboBox::new(
            &wnd,
            gui::ComboBoxOpts {
                position: POINT::new(20, 40),
                width: 140,
                ..Default::default()
            },
        );

        let rad_editor = gui::RadioGroup::new(
            &wnd,
            &[
                gui::RadioButtonOpts {
                    text: "Vim".to_owned(),
                    position: POINT { x: 20, y: 100 },
                    ..Default::default()
                },
                gui::RadioButtonOpts {
                    text: "Neovim".into(),
                    position: POINT { x: 20, y: 120 },
                    ..Default::default()
                },
                gui::RadioButtonOpts {
                    text: "Vscode".into(),
                    position: POINT { x: 20, y: 140 },
                    ..Default::default()
                },
            ],
        );

        let new_self = Self {wnd, lbl_title, cmb_lang, rad_editor};
        new_self.events();
        new_self
    }

    fn events(&self) {
        self.wnd.on().wm_create({
            let self2 = self.clone();
            move |_| {
                self2.cmb_lang.items().add(&["Rust", "Javascript", "Typescript", "Lua", "Go"]);
                self2.rad_editor[1].select(true);
                Ok(0)
            }
        });

        self.cmb_lang.on().cbn_sel_change({
            let self2 = self.clone();
            move || {
                if let Some(proguramming_language) = self2.cmb_lang.items().selected_text() {
                    self2.wnd.hwnd().SetWindowText(&proguramming_language)?;
                }
                Ok(())
            }
        });

        self.rad_editor.on().bn_clicked({
            let self2 = self.clone();
            move || {
                if let Some(selected_radio) = self2.rad_editor.checked() {
                    let the_editor = selected_radio.hwnd().GetWindowText()?;
                    self2.wnd.hwnd().SetWindowText(&the_editor)?;
                }
                Ok(())
            }
        });
    }
}

fn main() {
    let my = MyWindow::new();
    if let Err(e) = my.wnd.run_main(None) {
        eprintln!("{}", e);
    }
}
