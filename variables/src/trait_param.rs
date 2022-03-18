// cargo script src/trait_param.rs

pub trait Summary {
    fn summarize(&self) -> String;
}

// item can only be a type that implements Summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// impl Trait above is sugar syntax, here is the real one (called trait bound) which does the same:
pub fn notify<T: Summary>(item: &T) {}

// forces both params to be of a type that impl the same trait
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// specify more than one trait
pub fn notify(item: &(impl Summary + Display)) {}

// specify more than one trait with trait bound
pub fn notify<T: Summary + Display>(item: &T) {}

// "where" syntax, consider the 2 syntax:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

fn main() {}
