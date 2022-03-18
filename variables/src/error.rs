// cargo script src/error.rs

use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

// progagating error by returning them
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// shortcut to propagate errors
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// even shorter
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// shorter than shorter!
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    {
        // panic!("crash and burn");
    }
    {
        // let v = vec![1, 2, 3];
        // v[99];
    }
    {
        // let f = File::open("hello.txt");
        // println!("{:?}", f);
        // let f = match f {
        //     Ok(file) => file,
        //     Err(error) => panic!("Problem opening the file: {:?}", error),
        // };
    }
    {
        // let f = File::open("hello.txt");
        // let f = match f {
        //     Ok(file) => file,
        //     Err(error) => match error.kind() {
        //         ErrorKind::NotFound => match File::create("hello.txt") {
        //             Ok(fc) => fc,
        //             Err(e) => panic!("Problem creating the file: {:?}", e),
        //         },
        //         other_error => {
        //             panic!("Problem opening the file: {:?}", other_error)
        //         }
        //     },
        // };
    }
    {
        // let f = File::open("hello.txt").unwrap_or_else(|error| {
        //     if error.kind() == ErrorKind::NotFound {
        //         File::create("hello.txt").unwrap_or_else(|error| {
        //             panic!("Problem creating the file: {:?}", error);
        //         })
        //     } else {
        //         panic!("Problem opening the file: {:?}", error);
        //     }
        // });
    }
    {
        // success or panic
        // let f = File::open("hello.txt").unwrap();
    }
    {
        // success or panic with custom error message
        // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
}
