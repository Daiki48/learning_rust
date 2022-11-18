use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("Starting dir : {}", path.display());

    let root = Path::new("/");
    assert!(env::set_current_dir(&root).is_ok());

    let path = env::current_dir()?;
    println!("Final dir : {}", path.display());

    Ok(())
}
