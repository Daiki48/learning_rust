fn main() {

    // mutを外すとエラーが発生する。
    // rustの変数は通常、不変なので。
    let mut x = 1;
    println!("x is {}", x); // 1

    x = 9;
    println!("x is {}", x); // 9

    x = x + 1;
    println!("x is {}", x); // 10

    {
        // シャドーイング
        let x = x * 2;
        println!("x is {}", x); // 20
    }

    x = x + 5;
    println!("x is {}", x); // 15

}
