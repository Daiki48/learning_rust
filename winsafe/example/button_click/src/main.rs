#![windows_subsystem = "windows"]

use winsafe::prelude::*;
use winsafe::{
    gui,
    POINT,
    SIZE,
};

#[derive(Clone)]
pub struct MyWindow {
    wnd: gui::WindowMain,
    btn_hello: gui::Button,
}

impl MyWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "My Window Title".to_owned(),
                size: SIZE { cx: 300, cy: 500 },
                ..Default::default()
            },
        );

        let btn_hello = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: "&Click me".to_owned(),
                position: POINT::new(20, 20),
                ..Default::default()
            },
        );

        let new_self = Self {wnd, btn_hello};
        new_self.events();
        new_self
    }

    fn events(&self) {
        self.btn_hello.on().bn_clicked({
            let wnd = self.wnd.clone();
            move || {
                wnd.hwnd().SetWindowText("Hello, World!")?;
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
