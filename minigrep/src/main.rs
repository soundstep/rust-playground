use minigrep::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        bunt::eprintln!("{$red}Problem parsing arguments: {}{/$}", err);
        process::exit(1);
    });
    if let Err(err) = minigrep::run(config) {
        bunt::eprintln!("{$red}Application error: {}{/$}", err);
        process::exit(1);
    }
}
