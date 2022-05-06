// cargo script ./src/test3.rs

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("now sending: {}", val);
        tx.send(val).unwrap();
        // println!("val is {}", val); // this creates an error as the value has moved
    });

    // recv blocks the main thread until something is received
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
