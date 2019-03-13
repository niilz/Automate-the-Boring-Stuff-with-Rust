fn main() {
    // CONDITIONS
    
    // Example 1
    let name = "Mary";
    let password = "swordfish";
    if name == "Mary" {
        println!("Hello Mary");
        if password == "swordfish" {
            println!("Access granted.");
        } else {
            println!("Wrong password");
        }
    }

    // Example 2
    let name = "Bob";
    if name == "Alice" {
        println!("Hi, Alice");
    } else {
        println!("Hello, stranger");
    }

    // Example 3
    let name = "Bob";
    let age = 4000;
    if name == "Alice" {
        println!("Hi, Alice");
    } else if age < 12 {
        println!("You are not Alice, kiddo.")
    } else if age > 2000 {
        println!("Unlike you, Alice is not an undead, immortal vampire.");
    } else if age > 100 {
        println!("You are not Alice, grannie.")
    }

    // Example 3 rearranged (with bug)
    let name = "Dracula";
    let age = 4000;
    if name == "Alice" {
        println!("Hi, Alice");
    } else if age < 12 {
        println!("You are not Alice, kiddo.")
    } else if age > 100 {
        println!("You are not Alice, grannie.")
    } else if age > 2000 {
        println!("Unlike you, Alice is not an undead, immortal vampire.");
    }

    // Example 4
    let name = "Bob";
    let age = 30;
    if name == "Alice" {
        println!("Hi, Alice");
    } else if age < 12 {
        println!("You are not Alice, kiddo.");
    } else {
        println!("You are neither Alice nor a kid.");
    }
}
