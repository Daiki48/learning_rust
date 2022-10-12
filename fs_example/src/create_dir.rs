use std::fs;
use colored::*;

fn get_dir() -> String {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).ok();
    return text.trim().parse().ok().unwrap();
}

pub fn create_dir() {
    let output = "Enter a directory name.".color("blue");
    println!("{}", &output);
    let dir_name = get_dir();
    
    match fs::create_dir(&dir_name) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }
    println!("Made {}", &dir_name);
}
