use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

// slightly simplified definition of the vec! macro

// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

// attribute-like macro

// #[route(GET, "/")]
// fn index() {

// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

// function-like macro

// let sql = sql!(SELECT * FROM posts WHERE id=1);

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
