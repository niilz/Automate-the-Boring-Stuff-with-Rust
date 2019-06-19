fn main() {
    // FORMATTING
    // As far as I know there are no Python-like functions
    // like rjust, ljust or center.
    // But we could build them with the formatting-macro!
    // (Docs about this topic: std::fmt)
    // (From the docs: Arguments are formatted with Python-like syntax,
    //  meaning that arguments are surrounded by {})

    // The third argument is made an Option-type. RUST does not yet
    // have a build-in "optional-argument" style. So now you have to always
    // give it three arguments. If the third (fill) is not used, provide: None
    fn rjust(text: &str, width: usize, fill: Option<char>) -> String {
        // Since there is no way that I know of, to pass the fill-argument
        // as a variable, I used this work-around:
        if let Some(fill) = fill {
            let fill_len = width - text.len();
            let fill_str: String = (0..fill_len).map(|_| fill).collect();
            fill_str + text
        } else {
            format!("{text:>width$}", text=text, width=width) // The $-sign indicates that the width is of type usize
        }
        
    }

    // Same principle for ljust (just the swapped):
    fn ljust(text: &str, width: usize, fill: Option<char>) -> String {
        if let Some(fill) = fill {
            let fill_len = width - text.len();
            let fill_str: String = (0..fill_len).map(|_| fill).collect();
            // here we needed some additional transformation to please the compiler
            // (only str (slices) can be added to Strings, not the other way round):
            text.to_string() + &fill_str
        } else {
            format!("{text:<width$}", text=text, width=width)
        }
    }

    println!("{}", rjust("Hello", 10, None));
    //      Hello
    println!("{}", rjust("Hello", 20, None));
    //                Hello
    println!("{}", rjust("Hello World", 20, None));
    //          Hello World
    println!("{}", ljust("Hello", 10, None));
    // Hello     |<-- string goes up to here
    println!("{}", rjust("Hello", 20, Some('*')));
    // ***************Hello
    println!("{}", ljust("Hello", 20, Some('*')));
    // Hello***************

    // The center function is also similar but puts fill on both sides
    fn center(text: &str, width: usize, fill: Option<char>) -> String {
        if let Some(fill) = fill {
            let fill_len = (width - text.len()) / 2;
            let fill_str: String = (0..fill_len).map(|_| fill).collect();
            fill_str.clone() + text + &fill_str
        } else {
            format!("{text:^width$}", text=text, width=width)
        }
    }

    println!("{}", center("Hello", 20, None));
    //       Hello        |<-- String goes until here
    println!("{}", center("Hello", 20, Some('=')));
    // =======Hello=======

    // IN USAGE:
    fn printPicnic(itemsHashMap: &HashMap<&str, u32>, leftWidth: usize, rightWidth: usize) {

        println!("{}", center("PICNIC ITEMS", leftWidth + rightWidth, Some('-')));
        
        for (k, v) in itemsHashMap.iter() {
            println!("{}", ljust(k, leftWidth, Some('.')) + &rjust(&v.to_string(), rightWidth, None));
        }
    }
    use std::collections::HashMap;
    let mut picnicItems = HashMap::new();
    picnicItems.insert("sandwiches", 4);
    picnicItems.insert("apples", 12);
    picnicItems.insert("cups", 4);
    picnicItems.insert("cookies", 800);

    printPicnic(&picnicItems, 12, 5);
    // --PICNIC ITEMS--
    // cups........    4
    // cookies.....  800
    // apples......   12
    // sandwiches..    4
    printPicnic(&picnicItems, 20, 6);
    // -------PICNIC ITEMS-------
    // cups................     4
    // cookies.............   800
    // apples..............    12
    // sandwiches..........     4

}
