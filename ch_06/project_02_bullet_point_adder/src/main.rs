extern crate clipboard2;

use clipboard2::{Clipboard, SystemClipboard};

fn main() {
    let clipboard = SystemClipboard::new().unwrap();
    
    let clip_input = clipboard.get_string_contents().unwrap();

    let lines: Vec<String> = clip_input.split("\n").map(|s| "*".to_owned() + s).collect();

    let clip_output = lines.join("\n");

    clipboard.set_string_contents(clip_output);

    println!("{}", clipboard.get_string_contents().unwrap());
}

// if this...
// Lists of animals
// Lists of aquarium life
// Lists of biologists by author abbreviation
// Lists of cultivars
//
// ...get's copied into the clipboard it outputs:
//
// *Lists of animals
// *Lists of aquarium life
// *Lists of biologists by author abbreviation
// *Lists of cultivars