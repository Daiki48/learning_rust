fn main() {
    // 数字ではありません。
    let guess: u8 = "11".parse().expect("Not a number");
    println!("guess is {}", guess); // 11

    // 足し算
    let sum = 10 + 20;
    println!("sum : {}", sum); // 30

    // 引き算
    let difference = 55 - 10;
    println!("difference : {}", difference); // 45

    // 掛け算
    let product = 30 * 2;
    println!("product : {}", product); // 60

    // division
    // 割り算
    let quotient = 23.8 / 11.4;
    println!("quotient : {}", quotient); // 2.087719 ~

    // 余りは、切り捨て
    let floored = 2 / 3;
    println!("floored : {}", floored); // 0

    // remainder
    // 余り
    let remainder = 2 % 3;
    println!("remainder : {}", remainder); // 2

    // 文字型
    // char
    // シングルクォーテーションはcharになる
    // ダブルクォーテーションは&strになる (文字列スライス型)
    let c = 'z'; // char
    let z = 'ℤ'; // char
    let heart_eyed_cat = '😻'; // char
    let a = "a"; // &str
    println!("c : {}", c);
    println!("z : {}", z);
    println!("heart_eyed_cat : {}", heart_eyed_cat);
    println!("a : {}", a);

    // タプル型
    let tup: (i32, f64, u8) = (500, 2.7, 1);
    println!("tup : {:?}", tup);
    // タプルから値を取る
    // この過程を「分配」という
    let (x, y, z) = tup;
    println!("x in tup : {}", x);
    println!("y in tup : {}", y);
    println!("z in tup : {}", z);
    // ドット記法で直接値を取ることも出来る
    let tup2: (i8, u32, f64) = (-1, 222, 3.19);
    let tup2_first = tup2.0;
    println!("tup2_first : {}", tup2_first);

    // 配列型
    // 配列は、ヒープよりもスタックにメモリを確保したいときに最適です。
    // または、固定長の配列に有効です。例えば、1年の月を格納する場合であれば配列が有効です。
    // なぜなら、1月から12月の12個の要素は、その後伸縮する必要がないため。
    let array = [1, 2, 3, 4, 5];
    println!("array : {:?}", array);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];
    println!("months : {:?}", &months);
    println!("April in months : {}", &months[3]);
    // 配列の型注釈はこのように書く
    // [i32; 5] これは、i32型の要素が5個あるという意味
    let arr: [&str; 2] = ["Daiki", "Bob"];
    println!("arr : {}", arr[0]);
    // また、下記のように書くと、3という要素が5つ入っている配列を意味する
    let ary = [3; 5];
    println!("ary : {:?}", ary);

}
