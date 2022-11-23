use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("数を当てましょう！"); // Guess the Number

    let secret_number: u32 = rand::thread_rng().gen_range(1..11); // 1から10までのランダムな数字
    // println!("秘密の数字 : {}", &secret_number); // 確認用のoutput

    // 正解するまで問題を連続で問い続けるようにloop処理
    loop {
        println!("予想を入力してください。"); // Please input your guess.

        // この時点ではまだString型だが
        // あとの処理で数値にする。
        let mut guess = String::new();

        // stdin関数は、ターミナルの標準入力へのハンドルを表す型である
        // std::io::Stdinのインスタンスを返す
        io::stdin()
        .read_line(&mut guess)
        .expect("行の読み込みに失敗しました。"); // Failed to read line
        
        // trimメソッドは、文字列の前後の空白をすべて削除する。
        // parseメソッドは、解析時にエラーが発生しやすいのでエラー処理を入れる。
        // expectの呼び出しでは、プログラムがクラッシュして終了していたが、
        // match式にしてエラー処理を行う。
        // これは、parseメソッドがResultを返すので、match式でOkかErrを返すようにしている。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // parseで数値と解析できたやつ
            // Errで、数値と解析できなかったやつを処理する。
            Err(_) => continue, // continueでloopの次の処理を実行するようにしている。
        };

        println!("予想した数 : {}", &guess); // Your guess : {}
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さい！"),
            Ordering::Greater => println!("大きい！"),
            Ordering::Equal => {
                println!("正解！！");
                break; // 正解したらプログラムを終了する。
            },
        }
    }
}

