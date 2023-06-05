use std::{error::Error, io, process};

use goodreads::Book;

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Book = result?;
        println!("{:?}", record)
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error running: {}", err);
        process::exit(1);
    }
}
