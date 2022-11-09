#![windows_subsystem = "windows"]

use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, Dioxus Desktop App" }
    ))
}

fn main() {
    dioxus_desktop::launch(app);
}
