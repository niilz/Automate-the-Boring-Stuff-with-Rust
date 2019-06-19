fn main() {
    // START_WITH and ENDS_WITH

    println!("{}", "Hello world!".starts_with("Hello"));
    // true
    println!("{}", "Hello world!".ends_with("world!"));
    // true
    println!("{}", "abc123".starts_with("abcdef"));
    // false
    println!("{}", "abc123".ends_with("12"));
    // false
    println!("{}", "Hello world!".starts_with("Hello world!"));
    // true
    println!("{}", "Hello world!".ends_with("Hello world!"));
    // true

    // JOIN and SPLIT
    let animals = ["cats", "rats", "bats"];
    println!("{}", animals.join(", "));
    // cats, rats, bats
    let parts = ["My", "name", "is", "Simon"];
    println!("{}", parts.join(" "));
    // My name is Simon
    println!("{}", parts.join("ABC"));
    // MyABCnameABCisABCSimon

    // In RUST split() need one parameter
    // (in this example it can be type char or type str)
    let sentence = "My name is Simon";
    println!("{:?}", sentence.split(" ").collect::<Vec<&str>>());
    // ["My", "name", "is", "Simon"]
    // If we would have collected this into the String...
    println!("{:?}", sentence.split(" ").collect::<String>());
    // The print-statement would have looked like:
    // "MynameisSimon"
    println!("{:?}", sentence.split('m').collect::<Vec<&str>>());
    // ["My na", "e is Si", "on"]
    //
    // Split an new lines:
    let spam = r#"Dear Alice,
How have you been? I am fine.
Ther is a container in the fridge
that is labeled "Milk Experiment".

Please do not drink it.
Sincerely,
Bob"#;
    println!("{:?}", spam.split('\n').collect::<Vec<&str>>());
    // ["Dear Alice,", "How have you been? I am fine.", "Ther is a container in the fridge", "that is labeled \"Milk Experiment\".", "", "Please do not drink it.", "Sincerely,", "Bob"]
}
