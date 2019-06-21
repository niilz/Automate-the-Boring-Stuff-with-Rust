// project_01_password_locker
// An insecure password locker program.
extern crate clipboard2;

use clipboard2::{Clipboard, SystemClipboard};
use std::collections::HashMap;
use std::{env, process};
fn main() {
    // Tipp: There is a handy way to insert several key-value pairs
    // in one go. You do so by collecting a vec of tuples:
    let passwords: HashMap<&str, &str> = vec!(
        ("email", "F7minlBDDuvMJuxESSKHFhTxFtjVB6"),
        ("blog", "VmALvQyKAxiVH5G8v01if1MLZF3sdt"),
        ("luggage", "12345"),
    ).into_iter().collect();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:
with cargo: cargo run -- account
as binary: project_01_password_locker account");
        process::exit(1);
    }

    let account: &String = &args[1];
    
    if passwords.contains_key(&account[..]) {
        let clipboard = SystemClipboard::new().unwrap();
        let password = passwords[&account[..]];
        clipboard.set_string_contents(password.to_string());
        println!("Password for {} copied to clipboard.", password);
    } else {
        println!("There is no account named {}", account);
    }
    // Run the program (with a valid password key) and then
    // try to paste the clip-board-value somewhere
    // for example in a text-file or browser.
    // You'll see that the value, which you have
    // just looked up in the hashmap will be pasted.

}
