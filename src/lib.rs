#![allow(dead_code)]

struct Person {
    name: String,
    age: u32,
}

struct Book {
    title: String,
    author: String,
    is_avaliable: bool,
}

struct Library {
    books: Vec<Book>,
}
