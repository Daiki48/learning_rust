fn animal(dog: &str) {
    if dog == "dog" {
        panic!("Dog is cuteee!");
    }

    println!("{} !!!!", dog);
}
fn main() {
    animal("cat");
    animal("dog");
    animal("hello");
}
