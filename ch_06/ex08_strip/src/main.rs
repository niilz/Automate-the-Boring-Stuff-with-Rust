fn main() {
    // STRIPPING WHITESPACE
    // (In RUST mostly called trimming)

    let spam = "    Hello World    ";
    println!("{}", spam.trim());
    // Hello World

    println!("{}", spam.trim_right());
    //     Hello World

    println!("{}", spam.trim_left());
    // Hello World    |<-- str goes until here

    let spam = "SpamSpamBaconSpamEggsSpamSpam";
    println!("{}", spam.trim_matches(|c| "ampS".contains(c)));
    // BaconSpamEggs
}
