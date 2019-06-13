// first we nee the hashmap datastructure
use std::collections::HashMap;

fn main() {
    // create a new mutable hashmap (key-value store)
    let mut myCat = HashMap::new();
    // insert entries
    myCat.insert("size", "fat");
    myCat.insert("color", "gray");
    myCat.insert("disposition", "loud");

    println!("{:?}", myCat);
    // {"color": "gray", "disposition": "loud", "size": "fat"}
    println!("{}", myCat["size"]);
    // fat
    println!("My cat hat {} fur.", myCat["color"]);
    // My cat hat gray fur.

    // also works with numbers as keys
    let mut spam = HashMap::new();
    spam.insert(12345, "Luggage Combination");
    spam.insert(42, "The Answer");
    println!("{:?}", spam[&12345]);
    // "Luggage Combination"

    // Like in Python HashMaps are unordered
    let spam = ["cats", "dogs", "moose"];
    let bacon = ["dogs", "moose", "cats"];
    println!("{}", spam == bacon);

    let mut spam = HashMap::new();
    spam.insert("name", "Zophie");
    spam.insert("species", "cat");
    spam.insert("age", "8");
    // false

    let mut bacon = HashMap::new();
    bacon.insert("species", "cat");
    bacon.insert("age", "8");
    bacon.insert("name", "Zophie");
    println!("{:?}", spam);
    // maybe {"name": "Zophie", "species": "cat", "age": "8"}
    println!("{:?}", bacon);
    // maybe {"species": "cat", "name": "Zophie", "age": "8"}
    println!("{}", spam == bacon);
    // true (same behaviour as in Python)

    // If key does not exist, the program panics
    // println!("{}", spam["color"]);
    // would spit out:
    // thread 'main' panicked at 'no entry found for key'
}
