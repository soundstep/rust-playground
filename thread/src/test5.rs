// cargo script ./src/test5.rs

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }
    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let builder = thread::Builder::new().name("foo".into());
            let handle = builder
                .spawn(move || {
                    let mut num = counter.lock().unwrap();
                    println!("will now update num: {}", num);
                    thread::sleep(Duration::from_millis(300));
                    *num += 1;
                })
                .unwrap();
            handles.push(handle);
        }

        for handle in handles {
            let thread = handle.thread();
            println!("  thread id: {:?}", thread.id());
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
