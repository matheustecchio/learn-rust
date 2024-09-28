fn main() {
    let s1: String = String::from("hello");
    let s2: String = s1;

    // !!! ERROR: value is moved to s2
    println!("{}, world!", s1);
}
