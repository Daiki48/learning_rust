use std::fs::File;  // ファイルの操作をするための標準クレート
use std::io::prelude::*;  // ファイル入出力を含む入出力処理に関する標準クレート
use std::error::Error; // エラー処理

// 構造体にして設定値をまとめる
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str>{
        if args.len() < 3 {
            // 引数の数が足りません
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self {query, filename})
    }
}

// main関数からparse_config関数として抽出したが、
// Config構造体のコンストラクタのnew関数として定義しなおす。
// そうすることで、今後インスタンスを生成する際にConfig::newで生成できる。
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//
//     Config { query, filename }
// }

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With Text :\n{}", contents);

    Ok(())

    // 下記をmain関数から抽出してきたが、抽出したのでエラー処理を追加できる。
    // 上記がエラー処理まで施したコード。
    // let mut f = File::open(config.filename).expect("File not found");
    //
    // let mut contents = String::new();
    // // ファイルの読み込み中に問題がありました。
    // f.read_to_string(&mut contents)
    // .expect("Something went wrong reading the file.");
    //
    // // テキストは \n{} です。
    // println!("With text:\n{}", contents);

}

