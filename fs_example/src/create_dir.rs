use std::fs;

pub fn create_dir() {
    // カレントディレクトリ直下にディレクトリを作成。
    // すでに存在する場合は "! AlreadyExists" と表示される。
    println!("mkdir test");
    match fs::create_dir("test") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }
}
