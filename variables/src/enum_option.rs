// cargo script src/enum_option.rs

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let some_number = Some(5);
    println!("some_number: {:?}", some_number);
    let some_string = Some("a string");
    println!("some_string: {:?}", some_string);
    let absent_number: Option<i32> = None;
    println!("absent_number: {:?}", absent_number);

    let five = Some(5);
    println!("five: {:?}", five);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);
}
