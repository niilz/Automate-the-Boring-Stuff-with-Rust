fn main() {
    // In RUST str, &str and String cannot be indexed
    // by a single index number. (Reason is the encoding)
    // but you can define a slice (even if that is just one character)
    let name = "Zophie";
    println!("{}", &name[0..1]);
    // Z
    println!("{}", &name[name.len() -2 .. name.len() -1]);
    // i
    println!("{}", &name[..4]);
    // "Zoph"
    println!("{}", name.contains("Zo"));
    // true
    println!("{}", name.contains('z'));
    // false
    println!("{}", !name.contains('p'));
    // false
    for i in name.chars() {
        println!("* * * {} * * *", i);
    }
    // * * * Z * * *
    // * * * o * * *
    // * * * p * * *
    // * * * h * * *
    // * * * i * * *
    // * * * e * * *

    // MUTABLE AND IMMUTABLE DATA TYPES
    // In RUST you have to declare which variables are mutable (using mut)
    // However Strings, str, and &str cannot be changed by indexing
    //
    // so something like the following would not compile:
    // let mut name = "Zophie a cat";
    // name[7] = "the";
    //
    // But you can concatinate slices
    let name = "Zophie a cat";
    // For some reason RUST needs the first/base item to be of type String
    // and the others need to be slices
    let newName = name[..7].to_string() + &"the" + &name[8..12];
    println!("{}", newName);
    // Zophie the cat
    //
    // you could also use the "format!" macro:
    let newName = format!("{}{}{}", &name[..7], "the", &name[8..12]);
    println!("{}", newName);
    // Zophie the cat

    // reassigning can be done to a mutable variable or through shadowing it
    // mutable:
    let mut eggs = [1, 2, 3];
    eggs = [4, 5, 6];
    println!("{:?}", eggs);
    // [4, 5, 6]
    //
    // shadowing:
    let eggs2 = [7, 8, 9];
    let eggs2 = [10, 11, 12];
    println!("{:?}", eggs2);
    // [10, 11, 12]

    // deleting elements with remove (only works on vec types)
    let mut eggs = vec![1, 2, 3];
    eggs.remove(2);
    eggs.remove(1);
    eggs.remove(0);
    println!("{:?}", eggs);
    // []
    eggs.push(4);
    eggs.push(5);
    eggs.push(6);
    println!("{:?}", eggs);
    // [4, 5, 6]

    // TUPLES
    // differ from vec, slice and array that they can have fields of different types
    // (indexing works with dot "." notation)
    let eggs = ("hello", 42, 0.5);
    println!("{}", eggs.0);
    //
    // as far as I know you can not get a tuple slice or range
    // so the following would not compile:
    // println!("{:?}", eggs[1:3]);
    // 
    // Also there is no len function for tuples unless you implement
    // length behaviour yourself.
    // won't compile:
    // println!("{}", eggs.len());
    //
    // Therefore RUST tuples can be modified if they are mutable
    let mut eggs = ("hello", 42, 0.5);
    // But only if you change the right type.
    // so this works:
    eggs.1 = 99;
    println!("{}", eggs.1);
    // 99
    //
    // but the following would not (int where a string belongs)
    // eggs.0 = 2;
    // println!("{}", eggs.0);
    // 
    // Same as in Python. If one item is in parantheses RUST thinks
    // it is just a value. Unless you add a trailing comma.
    let mini = (32);
    println!("{}", mini);
    // 32
    let mini = (32,);
    println!("{:?}", mini);
    // (32,)

    // CONVERTING TYPES (kind of)
    // Vec to tuple:
    let vec = ["cat", "dog", "fish"];
    // First of all a vec or array or slice are all one type.
    // Secondly you would have to know the amount of values to unpack.
    let tuple = match vec {
        [val1, val2, val3] => (val1, val2, val3),
        _ => unreachable!(),
    };
    println!("{:?}", tuple);
    // ("cat", "dog", "fish")
    //
    // tuple to Vec:
    // very similar and again: onls works if the types are all the same.
    let tuple = (1, 2, 3);
    let vec = match tuple {
        (v1, v2, v3) => vec!(v1, v2, v3),
        _ => unreachable!(),
    };
    println!("{:?}", vec);
    // [1, 2, 3]
    // BTW even if you declare a Vec with vec!()
    // instead of vec![] it is still a vec type. The macro
    // does not care about the braces.

    // REFERENCES and COPIES
    let spam = String::from("Hello");
    // a reference is taken with the & (ampersand) symbol
    let cheese = &spam;
    println!("{} {}", cheese, spam);
    // Hello Hello
    //
    // simple integers and str (string slices) are actually copied
    // and are stored on the stack. So you don't have to reference them.
    let int = 42;
    let copy_of_int = int;
    println!("{} {}", int, copy_of_int);
    // 42 42
    //
    // if you would try that with a string or a vec (so without a reference)
    // the value is now moved into/ owned by the new variable.
    let vec = vec!(1, 2, 3);
    let new_vec_binding = vec;
    println!("{:?}", new_vec_binding);
    // [1, 2, 3]
    // this won't work (vec is consumed by the new_vec_binding,
    // or vec has been moved):
    // println!("{:?}", vec);
    //
    // If you make a simple type mutable, the copy won't be effected
    // if you change the first value.
    let mut int = 42;
    let int_copy = 42;
    int = 99;
    println!("{} {}", int, int_copy);
    // 99 42
    let mut string = String::from("fixed");
    let string_ref = &string;
    string = String::from("changed");
    // You cannot print string_ref now, because of the
    // assignment to string. So the following won't compile:
    // println!("{} {}", string, string_ref.to_string());
    //
    // you would have to copy or tranform the ref to an expandable type like vec
    let mut vec = vec![1, 2, 3, 4];
    let vec_copy = vec.clone();
    let vec_ref_to_vec = &vec.to_vec();
    vec = vec![6, 7, 8, 9];
    println!("{:?} {:?} {:?}", vec, vec_copy, vec_ref_to_vec);
    //[6, 7, 8, 9] [1, 2, 3, 4] [1, 2, 3, 4]
    //
    // So this way you can not really change references under your feet.
    // Meaning reassignments don't change other variables.
    // Which is different from Python or JavaScript.

    // FUNCTION PARAMETERS
    // It can happen that if you pass a mutable reference to a
    // function, and you change it, the actual values get changed.
    let mut vecky = vec!("hi", "I", "live", "outside");
    fn print_a_new_vec(vecky: &mut Vec<&str>) {
        vecky[3] = "inside";
        vecky.push("of");
        vecky.push("a");
        vecky.push("function");
        println!("{:?}", vecky);
    }
    print_a_new_vec(&mut vecky);
    // ["hi", "I", "live", "inside", "of", "a", "function"]
    println!("{:?}", vecky);
    // ["hi", "I", "live", "inside", "of", "a", "function"]
    // this should have been ["hi", "I", "live", "outside"]
    //
    // To avoid that, it is better to just borrow the vec and
    // return or print a new one.
    let vecky2 = vec!("second", "chance");
    // we use a ref of vecky2 so vecky2 still exists after the
    // function has run.
    fn new_vec<'a>(vecky: &'a Vec<&'a str>) -> Vec<&'a str> {
        let mut vec = vecky.clone(); // here the copy happens
        vec[1] = "working";
        vec.push("example");
        vec.to_vec()
    }
    let new_vecky2 = new_vec(&vecky2);
    println!("{:?}", vecky2);
    // ["second", "chance"]
    println!("{:?}", new_vecky2);
    // ["second", "working", "example"]
}