use std::fs;

fn main() {
    println!("`mkdir sample_dir`");
    // ディレクトリの作成
    match fs::create_dir("sample_dir") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => println!("Successful create directory!"),
    }
}
