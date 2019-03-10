fn main() {

    //Example 1
    println!("My name is");
    for i in 0..5 {
        println!("Jimm Five Times ({})", i.to_string());
    }

    // Example 2 (little Gauss)
    let mut total = 0;
    for num in 0..101 {
        total += num;
    }
    println!("{}", total);

    // Example 3 (Example 1 with "while")
    let mut i = 0;
    while i < 5 {
        println!("Jimmy Five Times ({})", i.to_string());
        i += 1;
    }

    // Example 4 (range with starting point)
    for i in (12..16) {
        println!("{}", i);
    }

    // Example 5 (range with starting step)
    for i in (0..10).step_by(2) {
        println!("{}", i);
    }

    // Example 6 (range but move backwards)
    // (notice the upper bound is not included,
    // also not if we go backwards. So to start at 5 we need to declare the end to be 6)
    for i in (0..6).rev() {
        println!("{}", i);
    }
}
