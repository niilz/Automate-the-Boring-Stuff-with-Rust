fn main() {
    // USING STRING METHODS TO VALIDATE USER INPUT

    // again I have the input-helper-function here:
    use std::io;
    fn input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.trim().to_string()
    }

    // Our own is_decimal-function (modified taking a String not &str now)
    fn is_decimal(s: String) -> bool {
        s.chars().all(|c| c.is_numeric())
    }
    // Our own is_alphanum-function (modified taking a String not &str now)
    fn is_alphanum(s: String) -> bool {
        s.chars().all(|c| c.is_alphanumeric())
    }

    loop {
        println!("Enter your age.");
        let age = input();
        if is_decimal(age) {
            break
        }
        println!("Please enter a number for your age.");
    }

    loop {
        println!("Select a new password (letters and numbers only)");
        let password = input();
        if is_alphanum(password) {
            break
        }
        println!("Password can only have letters and numbers.")
    }
}
