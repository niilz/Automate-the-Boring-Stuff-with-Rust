fn main() {
    use std::io;
    // here I just wrote a littel helper-function for the
    // input below.
    fn input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.trim().to_string()
    }

    // STRING METHODS
    let spam = "Hello World";
    let spam = spam.to_uppercase();
    println!("{}", spam);
    // HELLO WORLD

    let spam = spam.to_lowercase();
    println!("{}", spam);
    // hello world

    println!("How are you?");
    let feeling = input();
    if feeling.to_lowercase() == "great" {
        println!("I feel great too.")
    } else {
        println!("I hope the rest of your day is good.");
    }

    // There only is a is_lowercase() and is_uppercase() for type char.
    // To mimic that we could implemt our own is_lower()
    // and is_upper() for string.

    fn is_lower(s: &str) -> bool {
        s.chars().all(|c| c.is_lowercase())
    }
    fn is_upper(s: &str) -> bool {
        s.chars().all(|c| c.is_uppercase())
    }

    let spam = "Hello World";
    println!("{}", is_lower(spam));
    // false
    println!("{}", is_upper(spam));
    // false
    println!("{}", is_upper("HELLO"));
    // true
    println!("{}", is_lower("abc12345"));
    // false
    // (This is different from python now. To get the same
    // behaviour we could only check for alphabetic characters)
    println!("{}", is_lower("12345"));
    // false
    println!("{}", is_upper("12345"));
    // false

    println!("{}", "Hello".to_uppercase());
    // HELLO
    println!("{}", "Hello".to_uppercase().to_lowercase());
    // hello
    println!("{}", "Hello".to_uppercase().to_lowercase().to_uppercase());
    // HELLO
    println!("{}", "Hello".to_lowercase());
    // hello
    println!("{}", is_lower(&"Hello".to_lowercase()));
    // true

    // RUST has more is_... functions. Again they are
    // only implemented for char types
    fn is_alpha(s: &str) -> bool {
        s.chars().all(|c| c.is_alphabetic())
    }
    println!("{}", is_alpha("hello"));
    // true
    println!("{}", is_alpha("hello123"));
    // false

    fn is_alphanum(s: &str) -> bool {
        s.chars().all(|c| c.is_alphanumeric())
    }
    println!("{}", is_alphanum("hello123"));
    // true
    println!("{}", is_alphanum("hello"));
    // true

    fn is_decimal(s: &str) -> bool {
        s.chars().all(|c| c.is_numeric())
    }
    println!("{}", is_decimal("abc1234"));
    // false
    println!("{}", is_decimal("1234"));
    // true

    fn is_space(s: &str) -> bool {
        s.chars().all(|c| c.is_whitespace())
    }
    println!("{}", is_space("    "));
    // true

    fn is_title(s: &str) -> bool {
        s.split(' ')
         .all(|word| word.chars().next().unwrap().is_uppercase() && word[1..].chars().all(|c| c.is_lowercase()))
    }
    println!("{}", is_title("This Is Title Case"));
    // true
    println!("{}", is_title("This Is Title Case 123"));
    // false (again different from python because we are
    // not taking care of the digits.)
    println!("{}", is_title("This Is not Title Case"));
    // false
    println!("{}", is_title("This Is NOT Title Case Either"));
    // false
}
