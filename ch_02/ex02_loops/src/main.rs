fn main() {
    // LOOPS


    // Example 1
    let mut spam = 0; // in RUST a variable needs to be mutable (mut) if we like to change it
    while spam < 5 {
        println!("Hello, world.");
        spam = spam + 1;
    }


    // Example 2 (Annoying While Loop)
    use std::io;

    // A binding that we write to from stdin (user input).
    let mut name = String::new();

    // Trim the whitespace or newlines from the input
    // before we check the condition
    while name.trim() != "your name" {

        // This clears the name-variable (because name works as a buffer).
        // Otherwise every user input would get appended to the buffer.
        // (We would never be able to get it equal to "your name")
        name.clear();
        println!("Please type your name.");

        // Appends the user input plus a new-line-character to "name".
        io::stdin().read_line(&mut name);
    }
    println!("Thank you");


    // Example 2 (with "break" statement)
    while true {
        println!("Please type your name.");
        // this time we don't need to clear name/buffer, because
        // name get's created anew on every iteration
        let mut name = String::new();
        io::stdin().read_line(&mut name);
        if name.trim() == "your name" {
            break;
        }
    }
    println!("Thank you");

    // Example 3 (continue)
    // RUST has an infinit loop called "loop" (instead of the "while true")
    loop {
        println!("Who are you?");

        let mut name = String::new();
        io::stdin().read_line(&mut name);
        if name.trim() != "Joe" {
            continue;
        }

        println!("Hello, Joe. What is your password? (It is a fish.)");

        let mut password = String::new();
        io::stdin().read_line(&mut password);

        if password.trim() == "swordfish" {
            println!("Access granted.");
            break;
        }
    }

    // Example 4
    // there is no such thing as "truthy" or "falsey" in RUST
    // But we can check if a binding/variable is empty or not
    let mut name = String::from("");
    while name.is_empty() {
        println!("Enter your name:");
        io::stdin().read_line(&mut name);
    }
    println!("How many guests will you have?");
    let mut numOfGuests = String::new();
    io::stdin().read_line(&mut numOfGuests);
    // the number conversion requires to trim whitespace and than unpack (unwrap) the result
    // (unwraping is needed because "parse" returns a "Result"-type,
    // that can be Ok(value) or Err("error message"))
    let numOfGuests_int = numOfGuests.trim().parse::<u32>().unwrap();
    // Since there is no truthy, we check if the value is not zero or even smaller
    if numOfGuests_int > 0 {
        println!("Be sure to have enough room for all your guests.");
    }
    println!("Done");
}
