fn main() {
    println!("This is [vec!] macro");
    let mut x = vec!["Bob", "Sam"];
    println!("x[0]is {}, x[1] is {} -> vector Length: {}.", &x[0], &x[1], x.len());

    println!("[Alis] is push");
    x.push("Alis");
    println!("Length: {}.", x.len());
}
