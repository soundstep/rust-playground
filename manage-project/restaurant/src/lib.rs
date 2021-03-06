// Different syntax:
// use std::fmt::Result;
// use std::io::Result as IoResult;
// use std::io::{self, Write};
// use std::{cmp::Ordering, io};
// use std::collections::*;

mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;
// can also do:
// pub use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
// use self::front_of_house::hosting::add_to_waitlist;

// re-exporting with pub:
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    // enum only need one pub
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}
