fn main() {
    // CHECK FOR SUBSTRINGS
    // There is no "in" keyword to do the checking in RUST
    // But you can use string_or_str.contains(substring-slice);
    println!("{}", "Hello World".contains("Hello"));
    // true
    println!("{}", "Hello".contains("Hello"));
    // true
    println!("{}", "Hello".contains("HELLO"));
    // false
    println!("{}", "spam".contains(""));
    // true
    println!("{}", !"cats and dogs".contains("cats"));
    // false
}
