fn main() {

    // !!!! THIS WON'T COMPILE !!!!
    let mut outer = 10;

    // takes a number, adds it to an outer variable called "outer"
    // and prints it to the console
    fn my_function(num: i32) {
        outer += num;
        println!("{}", outer);
    }

    // should print (15)
    my_function(5);
    // but it does NOT. This would require a static outer variable and unsafe code
    // I don't fully understand those implications therefore won't go into it.

    // The easy way is to pass a variable into the function.
    // Where as long a you don't shadow it with a "let" statement
    // you always refer to that variable in scope
}
