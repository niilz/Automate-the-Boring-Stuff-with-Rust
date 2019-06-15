use std::collections::HashMap;

fn main() {
    // NESTED HashMaps
    let mut allGuests = HashMap::new();

    let mut alice = HashMap::new();
    alice.insert("apples", 5);
    alice.insert("pretzels", 12);

    let mut bob = HashMap::new();
    bob.insert("ham sandwiches", 3);
    bob.insert("apples", 2);

    let mut carol = HashMap::new();
    carol.insert("cups", 3);
    carol.insert("apple pies", 1);

    allGuests.insert("Alice", alice);
    allGuests.insert("Bob", bob);
    allGuests.insert("Carol", carol);
    // Nesting HashMaps is not as obvious as the dict literal of Python,
    // but it works aswell.
    println!("{:#?}", allGuests);
    // {
    //     "Carol": {
    //         "cups": 3,
    //         "apple pies": 1
    //     },
    //     "Alice": {
    //         "apples": 5,
    //         "pretzels": 12
    //     },
    //     "Bob": {
    //         "apples": 2,
    //         "ham sandwiches": 3
    //     }
    // }

    // COUNTING THE ITEMS
    // The guests variable has to be passed into the function as
    // a reference (&HashMap<...>).
    // Otherwise the HashMap could only be passed to one single function call,
    // because it gets consumed. Now it gets only borrowed.
    fn totalBrought(guests: &HashMap<&str, HashMap<&str, u8>>, item: &str) -> u8 {
        let mut numBrought = 0;
        for (k, v) in guests.iter() {
            numBrought += v.get(item).unwrap_or(&0u8);
        }
        numBrought
    }

    println!("Number of things being brought:");
    println!(" - Apples: {}", totalBrought(&allGuests, "apples"));
    println!(" - Cups: {}", totalBrought(&allGuests, "cups"));
    println!(" - Cakes: {}", totalBrought(&allGuests, "cakes"));
    println!(" - Ham Sandwiches: {}", totalBrought(&allGuests, "ham sandwiches"));
    println!(" - Apple Pies: {}", totalBrought(&allGuests, "apple pies"));
    //
    // Number of things being brought:
    // - Apples: 7
    // - Cups: 3
    // - Cakes: 0
    // - Ham Sandwiches: 3
    // - Apple Pies: 1
}
