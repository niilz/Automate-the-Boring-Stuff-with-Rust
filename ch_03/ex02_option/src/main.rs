fn main() {
    // RUST does not have nil, None, null or undefined.

    // It has a none-ish value which is the "union-type". It get's returned automatically
    // if a function does not return anything explicitly
    // The union-value looks like this: () 

    let spam = println!("hello"); // println! get's executed an writes "hello" to the console

    println!("{:?}", spam); // print's ()

    println!("{}", spam == ()); // print's true
}

    // RUST also has a variant called "Option", which can have two states.
    // The "Option-type" either has something inside, that can be "unwraped"
    // or it is in a "None" state, and you decide what to do in that case.
