use terminal_menu::{
    menu,
    list,
    run,
    button,
    label
};

pub fn list_menu() {
    std::process::Command::new("clear").status().unwrap();
    let lang = "rust";
    let menu = menu(vec![
        label("list menu"),
        list("li", vec![lang, "typescript", "go", "ruby"]),
        button("exit")
    ]);
    run(&menu);
}
