use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // たとえば、ここで cargo run -- rust javascript とコマンドを実行すると
    // ["target\\debug\\args_minigrep.exe", "rust", "javascript"]
    // という結果が出てくる
    // 一つ目に入ってるのはバイナリです。
    // println!("{:?}", args);

    // args[0]は、バイナリなので、下記のようにargs[1]とargs[2]を取る。
    let query = &args[1];
    let filename = &args[2];

    // cargo run -- test sample.txtとコマンドを打って確認してみる。

    println!("Searching for {}", query);
    println!("In file {}", filename);

}
