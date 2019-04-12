fn main() {
    // ther is no real list in python
    // but there are arrays, vectors and slices

    // all elements withing one of those data structures,
    // have to be the same
    let array = [1, 2, 3]; // fixed sice: stack allocated
    let vector = vec!["cat", "bat", "rat", "elephant"]; // growable: heap allocated
    let slice = &["hello", "and", "more", "strings"]; // reference to an array

    println!("{:?}", array);
    println!("{:?}", vector);
    println!("{:?}", slice);

    let spam = vector;
    println!("{}", spam[0]); // "cat";
    println!("{}", spam[1]); // "bat";
    println!("{}", spam[2]); // "rat";
    println!("{}", spam[3]); // "elephant";

    let hi = "hello ".to_string() + spam[0];
    println!("{}", hi); // hello cat
    // or
    let hi2 = format!("hello {}", spam[0]);
    println!("{}", hi2); // hello cat

    // DOESN'T compile:
    // let spam = [["cat", "bat"], [10, 20, 30, 40, 50]];
    // because of different types AND different lengths
    // println!("{:?}", spam);
    
    // this works:
    let spam = vec![vec![1, 2, 3], vec![10, 20, 30, 40, 50]];
    println!("{}", spam[1][4]); // 50

    let spam = vec!["cat", "bat", "rat", "elephant"];
    // negative indexing doesn't exist:
    // println!("{}", spam[-1]);

    // but this you can do:
    println!("{}", spam.last().unwrap()); // rat
    // or
    println!("{}", spam[spam.len() - 1]); // rat

    // SLICING:
    println!("{:?}", &spam[0..4]); // ["cat", "bat", "rat", "elephant"]
    println!("{:?}", &spam[1..3]); // ["bat", "rat"]
    println!("{:?}", &spam[0..spam.len() - 1]); // ["cat", "bat", "rat"]

    // SLICING SHORT-CUTS
    println!("{:?}", &spam[..2]); // ["cat", "bat"]
    println!("{:?}", &spam[1..]); // ["bat", "rat", "elephant"]
    println!("{:?}", &spam[..]); // ["cat", "bat", "rat", "elephant"]

    // getting the Lenght
    let spam = ["cat", "dog", "moose"];
    println!("{}", spam.len()); // 3

    // changing values at indexes:
    // (also works with vectors)
    // But the array or vector has to be mutable (mut)
    let mut spam = ["cat", "bat", "rat", "elephant"];
    spam[1] = "aardvark";
    println!("{:?}", spam); // ["cat", "aardvark", "rat", "elephant"]

    // either use "extend"
    let mut a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    a.extend(&b);
    println!("{:?}", a); // [1, 2, 3, 4, 5, 6]
    
    // remove an item by index (if vector is mutable)
    a.remove(2);
    println!("{:?}", a); // [1, 2, 4, 5, 6]
    a.remove(2);
    println!("{:?}", a); // [1, 2, 5, 6]
}
