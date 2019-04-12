fn main() {
    // other stuff with vectors

    //LOOPS
    for i in 0..4 {
        println!("{}", i);
    }
    // prints:
    // 0
    // 1
    // 2
    // 3

    // ...is the same as:
    for i in [0, 1, 2, 3,].iter() {
        println!("{}", i);
    }

    let supplies = ["pens", "staplers", "flame-throwers", "binders"];
    for i in 0..supplies.len() {
        println!("Index {} in supplies is: {}.", i, supplies[i]);
    }
    // prints:
    // Index 0 in supplies is: pens.
    // Index 1 in supplies is: staplers.
    // Index 2 in supplies is: flame-throwers.
    // Index 3 in supplies is: binders.

    // CHECK IF SOMETHING IS IN A VECTOR OR SLICE OR ARRAY
    println!("{}", ["hello", "hi", "howdy", "heyas"].contains(&"howdy"));
    // true

    let spam_array = ["hello", "hi", "howdy", "heyas"];
    println!("{}", spam_array.contains(&"cat"));
    // false
    let spam_slice = &["hello", "hi", "howdy", "heyas"];
    println!("{}", spam_slice.contains(&"cat"));
    // false
    let spam_vector = vec!["hello", "hi", "howdy", "heyas"];
    println!("{}", spam_vector.contains(&"cat"));
    // false

    // negated
    println!("{}", !spam_array.contains(&"cat"));
    // true
}
