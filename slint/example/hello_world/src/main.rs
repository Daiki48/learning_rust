slint::slint!{
    Helloworld := Window {
        Text {
            text: "Hello World";
            color: green;
        }
    }
}

fn main() {
    Helloworld::new().run();
}
