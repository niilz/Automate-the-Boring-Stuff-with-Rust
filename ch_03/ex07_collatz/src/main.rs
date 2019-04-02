fn main() {
    fn collatz(num: i32) -> i32 {
        match num % 2 {
            0 => {
                let res = num / 2;
                println!("{}", res);
                res
            },
            _ => {
                let res = 3 * num + 1;
                println!("{}", res);
                res
            }
        }
    }

    collatz(5);
    collatz(6);

    use std::io;

    fn collatz_sequence() {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let mut num = match input.trim().parse::<i32>() {
                Ok(num) => num,
                Err(e) => {
                    println!("{}, Please type an integer", e);
                    collatz_sequence();
                    1
                },
            };
        
        loop {
            if num == 1 { break; }
            num = collatz(num);
        }
    }

    collatz_sequence();
}
