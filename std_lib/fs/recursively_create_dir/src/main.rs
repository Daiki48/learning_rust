use std::fs;

fn main() {
    println!("mkdir -p dir1/dir2/dir3");
    fs::create_dir_all("dir1/dir2/dir3").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
