use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// let example_closure = |x| x;
// let s = example_closure(String::from("hello"));
// closure can use values from the outer scope, and values can aslso be moved to a closure
// let equal_to_x = move |z| z == x;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    // note: try to implement HashMap<u32, G>
    // so that the HashMap value can be other types like String
    store: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            store: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        self.store
            .entry(arg)
            .or_insert_with(|| (self.calculation)(arg))
            .clone()
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    println!("Will start");
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);
        let v3 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        assert_eq!(v2, v3);
    }
}
