use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    count: i32,
}

pub enum AppEvent {
    Increment,
    Decrement,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => {
                self.count += 1;
            }
            AppEvent::Decrement => {
                self.count -= 1;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData {count: 0}.build(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx| Label::new(cx, "+"));

            Label::new(cx, AppData::count).width(Pixels(50.0));

            Button::new(cx, |cx| cx.emit(AppEvent::Decrement), |cx| Label::new(cx, "-"));
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(50.0));
    })
    .title("Counter - App")
    .inner_size((400, 100))
    .run();
}
