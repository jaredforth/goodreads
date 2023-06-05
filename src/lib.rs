use serde::Deserialize;
use std::error::Error;

/// Rust representation of Goodreads book data
#[derive(Debug, Deserialize)]
pub struct Book {
    #[serde(rename = "Book Id")]
    pub book_id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Author")]
    pub author: String,
    #[serde(rename = "Author l-f")]
    pub author_l_f: String,
    #[serde(rename = "Additional Authors")]
    pub additional_authors: String,
    #[serde(rename = "ISBN")]
    pub isbn: String,
    #[serde(rename = "ISBN13")]
    pub isbn13: String,
    #[serde(rename = "My Rating")]
    pub my_rating: String,
    #[serde(rename = "Average Rating")]
    pub average_rating: String,
    #[serde(rename = "Publisher")]
    pub publisher: String,
    #[serde(rename = "Binding")]
    pub binding: String,
    #[serde(rename = "Number of Pages")]
    pub number_of_pages: String,
    #[serde(rename = "Year Published")]
    pub year_published: String,
    #[serde(rename = "Original Publication Year")]
    pub original_publication_year: String,
    #[serde(rename = "Date Read")]
    pub date_read: String,
    #[serde(rename = "Date Added")]
    pub date_added: String,
    #[serde(rename = "Bookshelves")]
    pub bookshelves: String,
    #[serde(rename = "Bookshelves with positions")]
    pub bookshelves_with_positions: String,
    #[serde(rename = "Exclusive Shelf")]
    pub exclusive_shelf: String,
    #[serde(rename = "My Review")]
    pub my_review: String,
    #[serde(rename = "Spoiler")]
    pub spoiler: String,
    #[serde(rename = "Private Notes")]
    pub private_notes: String,
    #[serde(rename = "Read Count")]
    pub read_count: String,
    #[serde(rename = "Owned Copies")]
    pub owned_copies: String,
}

/// Get a `Vec<Book>` from a path to a CSV
///
/// ## Usage:
///
/// ```
///  let books = goodreads::books_from_csv("csv/goodreads_library_export.csv".to_string());
///
///  assert!(books.is_ok());
///  assert_eq!(books.unwrap()[0].book_id, "25008661");
/// ```
pub fn books_from_csv(csv_path: String) -> Result<Vec<Book>, Box<dyn Error>> {
    let mut books = Vec::new();
    let mut rdr = csv::Reader::from_path(csv_path)?;
    for result in rdr.deserialize() {
        let record: Book = result?;
        books.push(record);
    }
    println!("{:?}", books);
    Ok(books)
}
