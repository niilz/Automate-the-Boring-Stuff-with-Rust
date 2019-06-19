extern crate clipboard2;

use clipboard2::{Clipboard, SystemClipboard};

fn main() {
    // CLIPBOARD EXERCISE (using extern crate clipboard2)


    let clipboard = SystemClipboard::new().unwrap();
    clipboard.set_string_contents(String::from("Hello World"));
    println!("{}", clipboard.get_string_contents().unwrap());
    // Hello World
    /* if I'd uncomment the set_string_contents line and would
       copy this senctence, the call to get_string_contents
       would look like the following: */

    // if I'd uncomment the set_string_contents line and would
    // copy this senctence, the call to get_string_contents
    // would look like the following:
}
