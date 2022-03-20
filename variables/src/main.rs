fn shadowing() {
    println!("--- main2");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn data_types() {
    println!("--- scalar types (integers, floating-point numbers, Booleans, and characters)");
    let var_integer: u32 = 12;
    let var_integer_signed: i32 = -12;
    let var_float: f64 = 10.12;
    let var_bool: bool = true;
    let var_char: char = 'C';
    println!("--- compound types (tuples and arrays)");
    let var_tuple: (i32, f64, u8) = (500, 6.4, 1);
    let var_array: [i32; 5] = [1, 2, 3, 4, 5];
}

fn control_flow() {
    println!("--- control flow: if");
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // one line
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("--- control flow: loop (loop, while and for)");
    {
        // 2 loops
        let mut count = 0;
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;
            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {}", count);
    }
    {
        // returning values
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {}", result);
    }
    {
        // while
        let mut number = 3;
        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }
        println!("LIFTOFF!!!");
    }
    {
        // for
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is: {}", element);
        }
        for number in (1..4).rev() {
            println!("{}!", number);
        }
    }
    {
        let s = String::from("Hello");
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                // do something
            }
        }
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope
    a_string // a_string is returned and moves out to the calling function
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    // str is the type for a string slice
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // <-- slice reference
        }
    }
    &s[..]
}

fn ownership() {
    {
        // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}, world!", s1); // <---- error here: borrow of moved value: `s1`, value borrowed here after move
        let s1 = String::from("hello");
        let s2 = s1.clone(); // s1 not removed from memory stack
        println!("s1 = {}, s2 = {}", s1, s2);
    }
    {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here
                            // println!("s = {}", s); // <--- move-borrow error
        let x = 5; // x comes into scope
        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    }
    {
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
    }
    {
        let mut s = String::from("hello");
        change(&mut s);
    }
    {
        // let mut s = String::from("hello");
        // let r1 = &s; // no problem
        // let r2 = &s; // no problem
        // let r3 = &mut s; // BIG PROBLEM
        // println!("{}, {}, and {}", r1, r2, r3);
    }
    {
        let my_string = String::from("hello world");

        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let word = first_word(&my_string);

        let my_string_literal = "hello world";

        // `first_word` works on slices of string literals, whether partial or whole
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
    }
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );
    shadowing();
    data_types();
    control_flow();
    ownership();
}
