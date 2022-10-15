use cursive::Cursive;
use cursive::views::Dialog;

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Did you do the thing?")
        .title("Question 1")
        .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
        .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
        .button("Uh?", |s| s.add_layer(Dialog::info("Try again!")))
    );
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
        .title("Results")
        .button("Finish", |s| s.quit())
    );
}

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(Dialog::text("This is a survey!\nPress <Next> when you're ready.")
    .title("Important survey")
    .button("Next", show_next));
    
    siv.run();
}

