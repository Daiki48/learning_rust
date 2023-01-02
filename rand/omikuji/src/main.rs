use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    let num: u8 = rng.gen();

    match num {
        0..=100 => println!("大吉"),
        101..=150 => println!("中吉"),
        151..=170 => println!("小吉"),
        171..=190 => println!("吉"),
        191..=200 => println!("末吉"),
        201..=220 => println!("凶"),
        221..=255 => println!("大凶"),
    }

    println!("乱数は、{}です。", &num);
}
