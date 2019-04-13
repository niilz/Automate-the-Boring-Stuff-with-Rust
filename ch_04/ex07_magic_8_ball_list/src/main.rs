fn main() {
    extern crate rand;
    use rand::seq::SliceRandom;

    let messages = ["It is certain",
        "It is decidedly so",
        "Yes definitely",
        "Reply hazy try again",
        "Ask again later",
        "Concentrate and ask again",
        "My reply is no",
        "Outlook not so good",
        "Very doubtful"];

    let mut rng = rand::thread_rng();
    println!("{:?}", messages.choose(&mut rng));
}
