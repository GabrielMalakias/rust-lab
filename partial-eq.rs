enum Format {
    Paperback,
    Hardback,
    Ebook
}

struct Book {
    isbn: i32,
    title: &'static str,
    format: Format
}

impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        self.isbn == other.isbn
    }
}

#[derive(PartialEq)]
struct MyStruct { member: i32 }

fn main() {
    let a = MyStruct { member: 1 };
    let b = MyStruct { member: 1 };

    let b1 = Book {
        isbn: 1234567890,
        title: "O Senhor dos Anéis",
        format: Format::Paperback
    };

    let b2 = Book {
        isbn: 1234567890,
        title: "O Senhor dos Anéis",
        format: Format::Paperback
    };

    let b3 = Book {
        isbn: 1234567810,
        title: "O Hobbit",
        format: Format::Hardback
    };


    println!("{}", a == b);

    println!("{}", b1 == b2);
    println!("{}", b2 == b3);
    println!("{}", b1 == b3);
}
