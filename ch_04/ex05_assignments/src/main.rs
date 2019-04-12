fn main() {
    // multiple Assignment tricks
    // or in RUST-words:
    // DESTRUCTURING

    let cat = ["fat", "orange", "loud"];
    let size = cat[0]; // fat
    let color = cat[1]; // orange
    let disposition = cat[2]; // loud

    // or
    let cat = ["fat", "orange", "loud"];
    let [size, color, disposition] = cat;
    println!("{}, {}, {}", size, color, disposition);
    // fat, orange, loud

    // swapping values:
    let [a, b] = ["Alice", "Bob"];
    let [b, a] = [a, b];
    println!("{}", a);
    // Bob
    println!("{}", b);
    // Alice

    // ADD AND ASSIGN
    let mut spam = 42;
    spam = spam + 1;
    println!("{}", spam); // 43

    // shorthand
    let mut spam = 42;
    spam += 1;
    println!("{}", spam); // 43

    spam -= 1; // spam = spam - 1
    println!("{}", spam); // 42
    spam *= 2; // spam = spam * 2
    println!("{}", spam); // 84
    spam %= 5; // spam = spam % 3
    println!("{}", spam); // 4
    spam /= 2; // spam = spam / 2
    println!("{}", spam); // 2

    // the add-and-assign technique does not work in RUST
    let mut spam = String::from("Hello"); // spam += "World"; is not possible
    // but you can push a string to it
    spam.push_str(" world!");
    println!("{}", spam);
    // Hello world!

    let bacon = ["Zophie"];
    // the following is not changing bacon, just shadowing it
    // and puts a string of 3*Zophie in an array
    let bacon = [bacon[0].repeat(3)];
    println!("{:?}", bacon); // ["ZophieZophieZophie"]

    // instead we coud map a over a range of 3,
    // and always return the first item of bacon
    let mut bacon = vec!["Zophie"]; // here we use a vector
    bacon = (0..3).map(|_| bacon[0]).collect(); // because map cannot be collected into an array
    println!("{:?}", bacon); // now we changed the vector
    // ["Zophie", "Zophie", "Zophie"]

}
