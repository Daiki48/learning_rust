extern crate minigrep;

use std::env;  // 引数など環境変数に関する標準クレート
use std::process;

use minigrep::Config;

fn main() {
    // collectで、引数すべてをVec<String>に変換
    let args: Vec<String> = env::args().collect();

    // unwrap_or_elseを使うことで、panic!ではない独自のエラー処理を定義できる。
    // 例えば、引数を1つだけ渡して実行すると
    // Problem parsing arguments : Not enough arguments
    // と出力される。
    // こちらのエラー処理の方がスリムで分かりやすい。
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {}", err);
        // process::exitは即座にプログラムを停止し、渡された数字を終了コードとして返す。
        process::exit(1);
    });
    // let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // リファクタリングで、下記ロジックをparse_config関数に抽出する。
    // let query = &args[1];    // argsはStringなので、&args[1]は&String
    // let filename = &args[2]; // こちらも同じく&String
    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    //
    // プロジェクトのカレントディレクトリからファイルを読み込む
    // ファイル名と、第二引数のファイル名が一致しなければパニックを起こす。
    // ファイルが見つかりませんでした。

    // run関数にエラー処理を追加したので、main関数も下記のように記述。
    if let Err(e) = minigrep::run(config) {
        println!("Application error : {}", e);
        process::exit(1);
    }

    // 以下の処理をrun関数に抽出する。
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
