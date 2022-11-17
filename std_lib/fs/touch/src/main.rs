use std::fs::OpenOptions;
use std::path::Path;

fn touch(path: &Path) -> std::io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("`touch c.txt`");
    touch(&Path::new("c.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
