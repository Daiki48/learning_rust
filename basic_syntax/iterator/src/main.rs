fn main() {
    // zipメソッド
    // 別のイテレータを受け取って合成して、新しいイテレータを返す。
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let zip_iter = a.iter().zip(b.iter());
    println!("{:?}", &zip_iter);
    // result -> Zip

    // mapメソッド
    let c = [1, 2, 3];
    let map_iter = c.iter().map(|x| 2 * x);
    println!("{:?}", &map_iter);
    // result -> Map {iter: Iter([1, 2, 3])}

}
