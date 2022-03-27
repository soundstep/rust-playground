#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;
use std::fs::read_to_string;
use std::io::stdout;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();
    println!("args {:?}", args);
    let content = read_to_string(&args.path).with_context(|| {
        format!(
            "could not read file `{}`",
            &args.path.into_os_string().into_string().unwrap()
        )
    })?;
    find_matches(&content, &args.pattern, &mut stdout());
    Ok(())
}
