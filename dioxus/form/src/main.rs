#![windows_subsystem = "windows"]

use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 { "Form" }
            form {
                onsubmit: move |ev| println!("Submitted {:?}", ev.values),
                input {r#type: "text", name: "username"}
                input {r#type: "password", name: "password"}
                input {r#type: "radio", name: "color", value: "red"}
                button {r#type: "submit", value: "Submit", "Submit the form"}
            }
        }
    })
}

fn main() {
    dioxus_desktop::launch(app);
}
