use std::collections::HashMap;
use std::io;

fn main() {
    // create the mutable key-value store (hasmap)
    let mut birthdays = HashMap::new();
    // insert items (here all String types)
    // (I've tried &str (slices) but ownership got in the way
    // because of the loop and it's scope)
    birthdays.insert(String::from("Alice"), String::from("Apr 1"));
    birthdays.insert(String::from("Bob"), String::from("Dec 12"));
    birthdays.insert(String::from("Luke"), String::from("May 5"));

    loop {
        println!("Enter a name: (blank to quit)");
        // get a name from the user
        let mut name = String::new();
        io::stdin().read_line(&mut name);
        // get rid of left and right whitespace
        name = name.trim().to_string();
        
        if name == "" {
            break;
        }

        if birthdays.contains_key(&name) {
            println!("{} is the birthday of {}", birthdays[&name], name);
        } else {
            println!("I do not have birthday information for {}", name);
            println!("What is their birthday?");
            // Get new B-Day for a name name not in the hashmap yet.
            // (same principle as above for name)
            let mut bday = String::new();
            io::stdin().read_line(&mut bday);
            let bday = bday.trim().to_string();
            // Put new name and B-Day into the hashmap.
            birthdays.insert(name, bday);
            println!("Birthday database updated.");
        }
    }
}
