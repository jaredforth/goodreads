use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Book {
    #[serde(rename = "Book Id")]
    pub id: String,
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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
