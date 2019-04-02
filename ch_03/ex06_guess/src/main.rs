fn main() {
    // This is a guess the number game.
    use rand::Rng;
    use std::io;

    let mut rand_gen = rand::thread_rng();
    let secret_number = rand_gen.gen_range(1, 21); // 21 not included
    println!("I am thinking of a number between 1 and 20");

    // Ask the player to guess 6 times
    for guessesTaken in 1..7 {
        println!("Take a guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess);

        let guess = guess.trim().parse::<i32>();
        match guess {
            Ok(guess) => {
                    if guess < secret_number {
                        println!("Your guess is to low");
                    } else if guess > secret_number {
                        println!("Your guess is to high");
                    } else {
                        println!("Good job You guessed my number in {} guesses!", guessesTaken);
                        return ();
                    } 
                },
                // if we use match we have to do something incase there is a
                // non usable value, so we handle that possible Error here
                Err(e) => println!("Please enter a number. The Error was: '{}'", e),
            }
    }
    println!("Nope. The number I was thinking of was {}.", secret_number);
}
