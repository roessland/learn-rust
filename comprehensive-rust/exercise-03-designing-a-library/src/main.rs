struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        Library { books: Vec::<Book>::new() }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{} ({})", book.title, book.year);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        if self.books.is_empty() {
            return None;
        }
        let mut oldest_book = &self.books[0];
        for book in &self.books {
            if book.year < oldest_book.year {
                oldest_book = book;
            }
        }
        Some(oldest_book)
    }

    fn oldest_book_fancy(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|book| book.year)
    }
}

// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
fn main() {
    let mut library = Library::new();

    println!("The library is empty: {}", library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!("The library is no longer empty: {}", library.is_empty());


    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    match library.oldest_book_fancy() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}

#[test]
fn test_library() {
    let mut library = Library::new();

    assert!(library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));

    library.add_book(Book::new("Blabla", 1950));


    assert!(!library.is_empty());

    assert_eq!(library.oldest_book().unwrap().year, 1950);

    library.print_books();

    assert_eq!(library.oldest_book().unwrap().year, 1950);

    assert_eq!(library.oldest_book_fancy().unwrap().year, 1950);
}