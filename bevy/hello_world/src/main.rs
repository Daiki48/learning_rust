use bevy::prelude::*;

fn hello_world() {
    println!("Hello World!!");
}

fn main() {
    App::new()
    .add_system(hello_world)
    .run();
}
