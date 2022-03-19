use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        bunt::eprintln!("{$red}Problem parsing arguments: {}{/$}", err);
        process::exit(1);
    });
    if let Err(err) = minigrep::run(config) {
        bunt::eprintln!("{$red}Application error: {}{/$}", err);
        process::exit(1);
    }
}
