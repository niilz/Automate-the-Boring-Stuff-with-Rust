fn main() {

    // There is a "try!" macro. But I couldn't fine out whether it could
    // be used to catch zero division.
    // Instead you can check explicitly for a zero division with
    // "checked_div()" which returns an Option
    // (btw. "checked_div" is not implemented for floats, so use integer values here)

    // You can than use this Option to Return a Result type,
    // if you wanted to show an Error with a message instead of just "None"

    fn spam(divide_by: i32) -> Result<i32, String> {
        match 42i32.checked_div(divide_by) {
            Some(value) => Ok(value),
            None => Err(String::from("please do not divide be zerooooo")),
        }
    }

    println!("{:?}", spam(2));  // Ok(21)
    println!("{:?}", spam(12)); // Ok(3)
    println!("{:?}", spam(0));  // Err("please do not...")
    println!("{:?}", spam(1));  // Ok(42)

    // If you would just divide by 0 without this kind of checking
    // the program would call the "panic!" macro and would crash.

    // EXAMPLE 2
    // to wrap the call to "println!("{:?}", spam(value))" we might have
    // to use a try-block. But this feature is as of typing under a feature flag
    // to get a similar behavior we could use a loop an break it if an error occurs

    // this function (no return value) takes a function-that-takes-a-number-and-returns-a-result
    // (exactly like spam above)
    fn spam_2(func: fn(i32) -> Result<i32, String>) {

        // we have a vector with 4 function calls
        let fn_vector = vec![
                            func(2),
                            func(12),
                            func(0),
                            func(1)
                        ];
        // iterate over the function-return-values in the vector
        // (their all either "Ok(value)" or "Err(message)" )
        for res in fn_vector.iter() {
            // we print the value or the Error-message
            // but if it is an Error -> we break the loop
            match res {
                Ok(value) => println!("{:?}", value),
                Err(e) => {
                    println!("{:?}", e);
                    break;
                },
            }
        }
    }

    spam_2(spam);
}
