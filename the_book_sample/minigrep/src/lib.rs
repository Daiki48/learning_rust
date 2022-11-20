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

    println!("Search word is {}", &config.query);
    println!("Search result : ");
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

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


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 空のVec配列を用意 （可変なベクタ）
    let mut results = Vec::new();

    // linesメソッド -> 文字列を行ごとに繰り返す。イテレータを返す。
    // line.contains(query) -> 現在の行がクエリ文字列を含むかどうかを確認する。
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    // forループの後に、results(ベクタ)を返却する。
    results
}

// 下記はテストで失敗させるためのsearch関数
// ずっと空のベクタ返すようになっている
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }

// testを実行するときは、まずは失敗するやつを行う
// そのあとに成功するやつを作る
// そのためのtest
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // rustは
        // 安全で速く生産性も高い。
        // 3つ選んで。
        let contents = "\
        Rust:
        // インデントするとtest実行時に左辺と右辺が一致しなくてエラーが発生する。
safe, fast, productive.
        Pick three.\
        ";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
