mod selections;
mod list;

use selections::select_menu;
use list::list_menu;

use terminal_menu::*;

fn main() {
    list_menu();
    // select_menu();
}
