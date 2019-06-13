fn main() {
    use std::collections::HashMap;

    let message = "It was a bright cold day in April, and the clocks were striking thirteen.";
    let mut count = HashMap::new();

    for character in message.chars() {
        *count.entry(character).or_insert(0) += 1;
    }
    println!("{:?}", count);
    // {'y': 1, 'k': 2, 'c': 3, ' ': 13, 'l': 3, 'h': 3, 'p': 1, 'd': 3, 'A': 1, ',': 1, 'n': 4, '.': 1, 'b': 1, 'i': 6, 'o': 2, 's': 3, 'r': 5, 't': 6, 'I': 1, 'w': 2, 'e': 5, 'g': 2, 'a': 4}
    //
    // To print in a more pretty way you can add a #
    // to the print function.
    println!("{:#?}", count);
//     {
//     'y': 1,
//     'k': 2,
//     'c': 3,
//     ' ': 13,
//     'l': 3,
//     'h': 3,
//     'p': 1,
//     'd': 3,
//     'A': 1,
//     ',': 1,
//     'n': 4,
//     '.': 1,
//     'b': 1,
//     'i': 6,
//     'o': 2,
//     's': 3,
//     'r': 5,
//     't': 6,
//     'I': 1,
//     'w': 2,
//     'e': 5,
//     'g': 2,
//     'a': 4
// }
}
