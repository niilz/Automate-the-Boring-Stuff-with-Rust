fn main() {
    
    let spam = ["hello", "hi", "howdy", "heyas"];
    println!("{}", spam.iter().position(|word| word == &"hello").unwrap());
    // 0
    println!("{}", spam.iter().position(|word| word == &"heyas").unwrap());
    // 3

    let spam = ["Zophie", "Pooka", "Fat-tail", "Pooka"];
    println!("{}", spam.iter().position(|word| word == &"Pooka").unwrap());
    // 1

    // Adding new values with push() and insert()
    let mut spam = vec!["cat", "dog", "bat"];
    spam.push("moose");
    println!("{:?}", spam);
    // ["cat", "dog", "bat", "moose"]

    let mut spam = vec!["cat", "dog", "bat"];
    spam.insert(1, "chicken");
    println!("{:?}", spam);
    // ["cat", "chicken", "dog", "bat"]

    // in RUST you CAN insert chars into strings
    let mut bacon = String::from("Hello");
    bacon.insert(1, 'X');
    println!("{:?}", bacon);
    // HXello

    // Remove items
    // // in nightly rust you can do:
    // let mut spam = vec!["cat", "bat", "rat", "elephant"];
    // spam.remove_item(&"bat");
    // println!("{:?}", spam);
    
    // on regular RUST you have to know the index
    let mut spam = vec!["cat", "bat", "rat", "elephant"];
    let index = spam.iter().position(|animal| animal == &"bat").unwrap();
    spam.remove(index);
    println!("{:?}", spam);
    // ["cat", "bat", "elephant"]

    // You could though:
    // create your own dataStructure that contains a Vector...
    #[derive(Debug)]
    struct MyVec<'a> {
        animals: Vec<&'a str>,
    };
    // ...and than you implement a remove-method for it
    impl<'a> MyVec<'a> {
        fn remove(&mut self, animal: &str) {
            let index = self.animals.iter().position(|a| a == &animal).unwrap();
            self.animals.remove(index);
        }
    }

    let mut spam = MyVec { animals: vec!["cat", "bat", "rat", "elephant"] };
    spam.remove(&"bat");
    println!("{:?}", spam.animals);
    // ["cat", "rat", "elephant"]

    // SORT
    let mut spam = [2, 5, 3, 1, -7];
    spam.sort();
    println!("{:?}", spam);
    // [-7, 1, 2, 3, 5]

    let mut spam = ["ants", "cats", "dogs", "badgers", "elephants"];
    spam.sort();
    println!("{:?}", spam);
    // ["ants", "badgers", "cats", "dogs", "elephants"]

    // sort_by always looks at the current and the next item
    // and swaps them according to the ordering rule you apply
    spam.sort_by(|a, b| a.cmp(&b)); // here: the same as "sort()"
    println!("{:?}", spam);
    // ["ants", "badgers", "cats", "dogs", "elephants"]
    //
    // to reverse the order:
    spam.sort_by(|a, b| b.cmp(&a));
    println!("{:?}", spam);
    // ["elephants", "dogs", "cats", "badgers", "ants"]

    // RUST also sorts uppercase items before lowercase
    let mut spam = ["Alice", "ants", "Bob", "badgers", "Carol", "cats"];
    spam.sort();
    println!("{:?}", spam);
    // ["Alice", "Bob", "Carol", "ants", "badgers", "cats"]

    // To order without all uppercase ending up in front
    // we can use "sort_by_key".
    // (because to_lowercase and to_uppercase return an iterator
    //  we have to collect the char again)
    let mut spam = ['a', 'z', 'A', 'Z'];
    spam.sort_by_key(|key| key.to_lowercase().collect::<Vec<char>>());
    println!("{:?}", spam);

}

