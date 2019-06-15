fn main() {
    // RUST has Strings and str types
    // String is growable and stored on the heap.
    // str is a String-slice, not growable and is stored on the stack.

    // here spam is actually a referenced string-slice (&str)
    let spam: &str = "That is Alice's cat.";
    println!("{}", spam);
    // That is Alice's cat.
    //
    // Escaping (\) the single quote.
    let spam: &str = "Say hi to Bob\'s mother.";
    println!("{}", spam);
    // Say hi to Bob's mother.

    println!("Hello there!\nHow are you?\nI\'m doing fine.");
    // Hello there!
    // How are you?
    // I'm doing fine.

    // RAW STRINGS
    println!(r"That is Carol\'s cat.");
    // That is Carol\'s cat. (The backslash did not get escaped)
    //
    // If one needs also quotes in the raw string, add # ont both sides.
    println!(r#"this is a "quoted" raw string."#);
    // this is a "quoted" raw string.

    // MULTI-LINE STRINGS
    println!("This is a \
        multi-line String. \
        If you add a backslash at the end \
        it is still treated as one \
        single line when it's printed.");
    // This is a multi-line String. If you add a backslash at the end it is still treated as one single line when it's printed.

    // MULTI-LINE COMMENTS
    // ... are done with /* ..text.. */
    /* so here is an example
        everything inbetween is
        treated as a comment */
}
