// This program says hello and asks for my name.

use std::io; // brings the module we need to access user input in scope 
fn main() {

    println!("Hello, world!");

    println!("What is your name?");
    let mut myName = String::new(); // declaring a new/empty String
    io::stdin().read_line(&mut myName); // putting the user input into "myName" (btw. no Error handling here)

    println!("It is good to meet you, {}", myName);
    println!("The length of you name is: {}", myName.len());

    println!("What is your age?");
    let mut myAge = String::new();
    io::stdin().read_line(&mut myAge);
    
    // the user input is a string. We first need to cut off the white-space or
    // new-line character using "trim()". Than parse it to an unsigned integer "u32"
    // (btw. also no error handling here on the "unwrap()" call)
    // The conversion back to "string" does the println!-macro on it's own
    println!("You will be {} in a year", myAge.trim().parse::<u32>().unwrap() + 1);
}
