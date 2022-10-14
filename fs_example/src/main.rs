mod create_dir;
mod recursively_create_dir;
mod metadata;

use create_dir::create_dir;
use recursively_create_dir::recursively_create_dir;
use metadata::meta_data;

fn get_variety() -> u32 {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).ok();
    return num.trim().parse().ok().unwrap();
}

fn main() {
    println!("Select the process to be executed.");
    println!("(1) create_dir");
    println!("(2) recursively_create_dir");
    println!("(3) metadata");

    let variety = get_variety();

    match &variety {
        1 => {
            println!("variety is create_dir");
            create_dir();
        },
        2 => {
            println!("variety is recursively_create_dir");
            recursively_create_dir();
        },
        3 => {
            println!("variety is metadata");
            // unused_must_use
            let _ = meta_data();
        },
        _ => println!("Not a covered option."),
    }
}

