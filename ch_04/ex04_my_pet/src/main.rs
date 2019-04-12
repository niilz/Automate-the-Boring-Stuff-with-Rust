fn main() {
    
    use std::io;
    
    let my_pets = ["Zophie", "Pooka", "Fat-tail"];
    println!("Enter a pet name:");

    let mut name = String::new();
    io::stdin().read_line(&mut name);

    if !my_pets.contains(&name.trim()) {
        println!("I do not have a pet named {}", name);
    } else {
        println!("{} is my pet.", name.trim());
    }
    
}
