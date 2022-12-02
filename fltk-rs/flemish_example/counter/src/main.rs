use flemish::{
    button::Button,
    color_themes,
    frame::Frame,
    group::Flex,
    prelude::*,
    OnEvent,
    Sandbox,
    Settings,
};

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Msg;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter flemish")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Msg::IncrementPressed => {
                self.value += 1;
            }
            Msg::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) {
        let col = Flex::default_fill().column();
        Button::default()
            .with_label("+")
            .on_event(Msg::IncrementPressed);
        Frame::default().with_label(&self.value.to_string());
        Button::default()
            .with_label("-")
            .on_event(Msg::DecrementPressed);
        col.end();
    }
}


fn main() {
    Counter::new().run(Settings {
        size: (300, 100),
        resizable: true,
        color_map: Some(color_themes::BLACK_THEME),
        ..Default::default()
    })
}
