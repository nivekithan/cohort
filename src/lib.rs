use std::{collections::HashMap, fmt::Display};

pub struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let age = &self.age;
        return write!(f, "{{ name: {name}, age: {age}}}",);
    }
}

impl Person {
    /**
     * Solution for: Create a function/method that allows borrowing a Person and displaying their name and age.
     */
    pub fn display_name_and_age(&self) {
        println!("{self}");
    }
}

pub struct Book {
    title: String,
    author: String,
    is_avaliable: bool,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = &self.title;
        let author = &self.author;
        let is_avaliable = &self.is_avaliable;

        return write!(
            f,
            "{{ title: {title}, author: {author}, is_avaliable: {is_avaliable} }}"
        );
    }
}
pub struct Library {
    books: Vec<Book>,
    books_to_borrowers: HashMap<String, String>,
}

impl Library {
    /**
     * Solution for: Implement a method in the Library struct to check out a book, changing its is_available status to false.
     * Ensure proper borrowing and ownership rules are followed
     */
    pub fn checkout(&mut self, book_title: &str, name: &str) -> Result<&Book, String> {
        let book = match self.books.iter_mut().find(|book| book.title == book_title) {
            Some(book) => book,
            None => return Err(format!("No book with tile: {book_title} found")),
        };

        match book.is_avaliable {
            false => return Err(format!("Book {book_title} is already checked out")),
            true => {
                book.is_avaliable = false;
                self.books_to_borrowers
                    .insert(book_title.to_string(), name.to_string());
                return Ok(book);
            }
        }
    }

    /**
     * Solution for: Create a method in the Library struct to list all available books
     */
    pub fn list_all_books(&self) {
        let mut list_of_books = String::new();

        self.books.iter().for_each(|book| {
            let formatted_book = format!("{book}");
            list_of_books.push_str(&formatted_book);
            list_of_books.push_str(",\n");
        });

        println!("[\n{list_of_books}\n]");
    }

    /**
     * Solution for: Write a method that lists all the books currently checked out, displaying the borrower's name
     * (if checked out).
     */
    pub fn list_checked_out_books(&self) {
        let mut books_with_borrowers_name = String::new();

        self.books.iter().for_each(|book| {
            let is_checked_out = book.is_avaliable;

            match is_checked_out {
                false => return (),
                true => {
                    let borrower_name = self
                        .books_to_borrowers
                        .get(&book.title)
                        .expect("[UNEXPECTED] Checkout book does not have a borrower");

                    books_with_borrowers_name
                        .push_str(&format!("book: {book},\nborrower: {borrower_name}\n\n"));
                }
            }
        });

        println!("{books_with_borrowers_name}");
    }
}

/**
 * Solution for: Write a function to return a book to the library, changing its is_available status to true.
 * Ensure proper borrowing and ownership rules are followed
 */
pub fn return_book_to_libary(book: &Book, library: &mut Library) -> Result<(), String> {
    let book_title = &book.title;

    let book_in_library = match library
        .books
        .iter_mut()
        .find(|book| &book.title == book_title)
    {
        Some(book) => book,
        None => return Err(format!("No book with title: {book_title} is found")),
    };

    match book_in_library.is_avaliable {
        true => return Err(format!("Book {book_title} has never been checked out")),
        false => {
            book_in_library.is_avaliable = true;
            library.books_to_borrowers.remove(book_title);
            return Ok(());
        }
    }
}
