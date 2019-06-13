use std::collections::HashMap;

fn main() {
    // RETRIEVE values with get()
    let mut picnicItems = HashMap::new();
    picnicItems.insert("apples", 5);
    picnicItems.insert("cups", 2);
    println!("I am bringing {:?} cups", picnicItems.get("cups"));
    // get in RUST returns a Result type (Some(val) or None)
    // so here the print statement looks like this:
    // I am bringing Some(2) cups
    //
    // But you can unwrap the value
    println!("I am bringin {} eggs", picnicItems.get("apples").unwrap());
    // I am bringin 5 eggs
    //
    // Or with a fallback using unwrap_or()
    println!("I am bringin {} eggs", picnicItems.get("eggs").unwrap_or(&0));
    // I am bringin 0 eggs
    //
    // just using bracket-notation [] might cause the progam
    // to panic if the key does not exist
    // println!("I am bringin {} eggs", picnicItems["eggs"]);
    // thread 'main' panicked at 'no entry found for key'

    // SETTING VALUES
    let mut spam = HashMap::new();
    spam.insert("name", "Pooka");
    spam.insert("age", "5");

    if !spam.contains_key("color") {
        spam.insert("color", "black");
    }
    //
    // You can do this in one line
    spam.entry("hobby").or_insert("singing");
    println!("{:?}", spam);
    // {"hobby": "singing", "name": "Pooka", "color": "black", "age": "5"}
    spam.entry("hobby").or_insert("reading");
    println!("{:?}", spam);
    // {"hobby": "singing", "name": "Pooka", "color": "black", "age": "5"}
}
