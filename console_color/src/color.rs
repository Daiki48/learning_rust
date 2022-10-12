use colored::*;

pub fn colors() {
    let hello_world = "Hello, world!";
    println!("{} is plane.\n",&hello_world);
    println!("{} is black.\n", &hello_world.black());
    println!("{} is red.\n", &hello_world.red());
    println!("{} is green.\n", &hello_world.green());
    println!("{} is yellow.\n", &hello_world.yellow());
    println!("{} is blue.\n", &hello_world.blue());
    println!("{} is magenta.\n", &hello_world.magenta());
    println!("{} is cyan.\n", &hello_world.cyan());
    println!("{} is white.\n", &hello_world.white());
}
