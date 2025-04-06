use std::error::Error;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use clap::Parser;

pub mod types;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // read file name
    #[arg(short, long)]
    pub file_name: String,
}

pub fn run_file(file_name: &String) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;

    run(file)?;

    Ok(())
}

fn run(file: File) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

pub fn error(line: usize, message: &str) {
    panic!("Error: {} on line {}", message, line);
}
