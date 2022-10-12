use terminal_menu::*;

pub fn select_menu() {
    let menu = menu(vec![
        label("lists and scrolls"),

        // list: 
        // show all values
        // surround the selected value with brackets
        list("li", vec![
                "Daiki",
                "Bob",
                "Som"
        ]),

        // scroll:
        // show only the selected item
        scroll("sc", vec![
                "Daiki",
                "Bob",
                "Som"
        ]),

        // terminal control is finish.
        button("exit")
    ]);
    run(&menu);
    {
        let mm = mut_menu(&menu);
        println!("{}", mm.selection_value("li"));
        println!("{}", mm.selection_value("sc"));
    }
}
