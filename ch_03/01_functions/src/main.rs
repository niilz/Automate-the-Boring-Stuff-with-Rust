fn main() {
    // FUNCTIONS

    // Example 1
    fn hello() {
        println!("Howdy!");
        println!("Howdy!!!");
        println!("Hello there.");
    }

    hello();
    hello();
    hello();

    
    // Example 2 (with arguments)
    fn hello_name(name: String) {
        println!("Hello {}", name);
    }

    hello_name(String::from("Alice"));
    hello_name(String::from("Bob"));


    // Example 3 magic 8-Ball
    extern crate rand;
    use rand::Rng;
    fn getAnswer(answerNumber: u32) -> String {
        if answerNumber == 1 {
            return String::from("It is certain")
        } else if answerNumber == 2 {
            return String::from("It is decidedly so")
        } else if answerNumber == 3 {
            return String::from("Yes")
        } else if answerNumber == 4 {
            return String::from("Reply hazy try again")
        } else if answerNumber == 5 {
            return String::from("Ask again later")
        } else if answerNumber == 6 {
            return String::from("Concentrate and ask again")
        } else if answerNumber == 7 {
            return String::from("My reply is no")
        } else if answerNumber == 8 {
            return String::from("Outlook not so good")
        } else if answerNumber == 9 {
            return String::from("Very doubtful");
        } else {
            return String::from("no such number");
        }
    }

    let r = rand::thread_rng().gen_range(1, 10);
    let fortune = getAnswer(r);
    println!("{}", fortune);

    // Example 3 (Magic 8 Ball with MATCH)

    fn getAnswer_match(answerNumber: u32) -> String {
        match answerNumber {
            1 => String::from("It is certain"),
            2 => String::from("It is decidedly so"),
            3 => String::from("Yes"),
            4 => String::from("Reply hazy try again"),
            5 => String::from("Ask again later"),
            6 => String::from("Concentrate and ask again"),
            7 => String::from("My reply is no"),
            8 => String::from("Outlook not so good"),
            _ => String::from("Very doubtful"),
        }
    }

    let r = rand::thread_rng().gen_range(1, 9);
    let fortune = getAnswer(r);
    println!("{}", fortune);
}
