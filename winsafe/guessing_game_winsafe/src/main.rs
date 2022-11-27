#![windows_subsystem = "windows"]

mod my_window;
use my_window::MyWindow;

fn main() {
    let mainwindow = MyWindow::new();
    if let Err(e) =mainwindow.wnd.run_main(None) {
        eprintln!("{}", e);
    }
}
