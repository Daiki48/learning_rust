use std::fs;

pub fn recursively_create_dir() {
    println!("mkdir -p a/b/c");
    fs::create_dir_all("a/b/c").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
