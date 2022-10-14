use std::fs;

pub fn meta_data() -> std::io::Result<()> {
    let metadata = fs::metadata("./data/foo.txt")?;

    println!("{:?}", metadata.len());
    // metadata.is_dir()
    // metadata.is_file()
    // metadata.file_type()
    // metadata.is_symlink()
    // metadata.len()
    Ok(())
}
