pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else if b > a {
        b
    } else {
        panic!("Both numbers are equal");
    }
}