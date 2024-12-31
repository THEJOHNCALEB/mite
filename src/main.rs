use clap::Parser;

#[derive(Parser)]
struct Mite {
    command: String,
    option: std::path::PathBuf,
}

fn main() {
    let mite_stuff = Mite::parse();
    let content = std::fs::read_to_string(&mite_stuff.option).expect("could not read file");
    for word in content.lines() {
        if word.contains(&mite_stuff.command) {
            println!("{}", word);
        }
    }
    println!("{:?} {:?}", mite_stuff.command, mite_stuff.option);
}