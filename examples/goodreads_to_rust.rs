use std::{error::Error, process};

fn run() -> Result<(), Box<dyn Error>> {
    let csv_path = String::from("csv/goodreads_library_export.csv");
    let books = goodreads::books_from_csv(csv_path);
    for book in books? {
        println!("{:?}", book)
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error running: {}", err);
        process::exit(1);
    }
}
