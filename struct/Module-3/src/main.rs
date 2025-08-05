use std::fs::File;
use std::io::{Write, BufReader, BufRead};

//setup the book struct
struct Book {
    title: String,
    author: String,
    year: u16,
}
fn main(){
    book_catalog();
    println!("Program ended.");
}

//Assignment: Book Catalog

//Function to save books to a file
fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    //iterate through the books vector and write each book's details to the file
    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year).expect("Unable to write to file");
    }
}

//function to load books from a file
fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut books = Vec::new();
    //iterates through each line in the file, using a buffered reader variable for efficiency
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        //split up the line by assuming commas. Each component is collected and stored in a vector. Then pushed to books.
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year: u16 = parts[2].parse().expect("Unable to parse year");
            books.push(Book { title, author, year });
        }
    }
    books
}

fn book_catalog() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}