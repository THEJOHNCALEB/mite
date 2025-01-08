use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Parser)]
struct Mite {
    command: String,
    option: PathBuf,
}

fn main() -> io::Result<()> {
    let mite_stuff = Mite::parse();
    let file = File::open(&mite_stuff.option).expect("could not open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(&mite_stuff.command) {
            println!("{}", line);
        }
    }
    println!("{:?} {:?}", mite_stuff.command, mite_stuff.option);
    Ok(())
}