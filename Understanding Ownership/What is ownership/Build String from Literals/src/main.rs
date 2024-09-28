fn main() {
    let part1: &str = "Rust ";
    let part2: &str = "is the ";
    let part3: &str = "greatest programming language";
    let mut message: String = String::from("I think ");
    message.push_str(part1); message.push_str(part2); message.push_str(part3);
    println!("{}", message)
}
