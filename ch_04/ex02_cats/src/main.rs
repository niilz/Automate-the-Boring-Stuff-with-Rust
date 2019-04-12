fn main() {
    use std::io;

    let mut catNames = Vec::new();

    loop {
        println!("Enter the name of cat {} (Or enter nothing to stop.):", catNames.len() + 1);

        let mut name = String::new();
        io::stdin().read_line(&mut name);

        if name.trim() == "" {
            break
        }
        catNames.push(name);
    }
    println!("The cat names are: ");
    for name in catNames.iter() {
        println!("{}", name);
    }
}
