fn main() {
    // INDEXING DOES NOT WORK DIRECTLY
    // (This has to do with the unicode implementation.
    // Not every character in every language or emojis
    // just take one index.)

    let spam = "hello world!";
    // So this won't compile:
    // println!("{}", &spam[3]);
    //
    // But you can take ranges, even if the only contain
    // a single character.
    println!("{}", &spam[0..1]); // h
    println!("{}", &spam[4..5]); // o
    println!("{}", &spam[0..5]); // hello
    println!("{}", &spam[..5]);  // hello
    println!("{}", &spam[6..]);  // !
    // To get the last character you can either take the
    // take the length - 1 until the end:
    println!("{}", &spam[spam.len() - 1..]); // !
    //
    // Or you use the last() method on an char-iterator of
    // the string which returns a result-type
    // that you can unwrap() afterwards.
    println!("{:?}", spam.chars().last()); // Some('!')

    let spam = "Hello world!";
    let fizz = &spam[..5];
    println!("{}", fizz);
    // Hello
}
