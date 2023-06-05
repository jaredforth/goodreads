use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Book {
    #[serde(rename = "Book Id")]
    pub id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Author")]
    pub author: String,
    #[serde(rename = "Date Read")]
    pub pubdate: String,
    #[serde(rename = "Bookshelves")]
    pub bookshelves: String,
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
