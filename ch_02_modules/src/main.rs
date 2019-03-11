fn main() {
    // MODULES (standard library)

    // Example 1 (Random integer)
    // Import modules from standard library with "use"
    use rand::Rng;

    // To be able to get a random number in a specific range,
    // we create a random-number-generator
    let mut rand_gen = rand::thread_rng();

    // And then use this generator
    for i in 0..5 {
        println!("{}", rand_gen.gen_range(1, 10));
    }

    // Example 2 (exit the program)
    use std::process;
    use std::io;

    // nothing new here
    loop {
        println!("Type exit to exit");

        let mut response = String::new();
        io::stdin().read_line(&mut response);

        if response.trim() == "exit" {
            // Here we use the the exit function to terminate the program
            process::exit(1);
        }
        println!("You typed {}.", response.trim());
    }
}
