use std::collections::HashMap;

fn main() {
    // hashmap methods
    let mut spam = HashMap::new();
    spam.insert("color", "red");
    spam.insert("age", "42");
    // 42 has to be str because it has to be the same type
    // as every other value in the hashmap
    // (otherwise you would have to implement a data
    //  structure yourself)
    //
    // for example like this:
    #[derive(Debug)]
    struct Spam {
        color: String,
        age: i32
    }
    let spam_struct = Spam {
            color: String::from("red"),
            age: 42,
        };
    println!("{:?}", spam_struct);
    // Spam { color: "red", age: 42 }
    // but this as an exact amount of fields and
    // also the next methods (values(), keys(), iter())
    // would not work until you implement them for this
    // particular datastructure

    // So, going on with HashMap (all values of same type):
    for v in spam.values() {
        println!("{}", v);
    }
    // red
    // 42

    for k in spam.keys() {
        println!("{}", k);
    }
    // age
    // color

    for t in spam.iter() {
        println!("{:?}", t);
    }
    // ("color", "red")
    // ("age", "42")

    // collect hashmap keys into a vec
    let keys: Vec<_> = spam.keys().collect(); // btw the type is Vec<&&str>
    println!("{:?}", keys);
    // ["age", "color"]

    for (k, v) in spam.iter() {
        println!("Key: {} Value: {}", k, v);
    }
    // Key: age Value: 42
    // Key: color Value: red

    // CHECKING if KEY or VALUE exists in a HASHMAP
    let mut spam = HashMap::new();
    spam.insert("name", "Zophie");
    spam.insert("age", "7");
    // check for a key:
    println!("{}", spam.contains_key("name"));
    // true
    //
    // check for a value:
    let vals: Vec<_> = spam.values().collect();
    println!("{:?}", vals.contains(&&"Zophie"));
    // or
    let vals: Vec<&str> = spam.iter().map(|(k, v)| *v).collect();
    println!("{:?}", vals.contains(&"Zophie"));
    // true
    //
    println!("{}", spam.contains_key("color"));
    // false
    println!("{}", !spam.contains_key("color"));
    // true
    // There is no "contains" function on HashMap 
    // the following would not compile:
    // println!("{}", spam.contains("color");
}
