mod tests {
    use std::cmp::Ordering;

    #[test]
    fn test_order_then_in_tuple() {
        let x: (i64, i64, i64) = (1, 2, 7);
        let y: (i64, i64, i64) = (1, 5, 3);
        let z: (i64, i64, i64) = (1, 2, 3);

        let result1 = x.0.cmp(&y.0).then(x.1.cmp(&y.1)).then(x.2.cmp(&y.2));
        assert_eq!(result1, Ordering::Less);

        let result1 = x.0.cmp(&z.0).then(x.1.cmp(&z.1)).then(x.2.cmp(&z.2));
        assert_eq!(result1, Ordering::Greater);
    }

    #[test]
    fn test_cmp() {
        #[derive(PartialEq)]
        enum BookFormat {
            HardBack,
            PaperBack,
            Ebook,
        }

        struct Book {
            isbn: u32,
            format: BookFormat,
        }

        impl PartialEq for Book {
            fn eq(&self, other: &Self) -> bool {
                self.isbn == other.isbn
            }
        }
        // Implement <Book> == <BookFormat> comparisons
        impl PartialEq<BookFormat> for Book {
            fn eq(&self, other: &BookFormat) -> bool {
                self.format == *other
            }
        }

        // Implement <BookFormat> == <Book> comparisons
        impl PartialEq<Book> for BookFormat {
            fn eq(&self, other: &Book) -> bool {
                *self == other.format
            }
        }

        let a_book = Book {
            isbn: 1,
            format: BookFormat::HardBack,
        };
        assert_eq!(a_book == BookFormat::HardBack, true); // see Implement <Book> == <BookFormat> comparisons
        assert_eq!(BookFormat::HardBack == a_book, true); // see Implement <BookFormat> == <Book> comparisons

        let b_book = Book {
            isbn: 2,
            format: BookFormat::HardBack,
        };
        let c_book = Book {
            isbn: 1,
            format: BookFormat::Ebook,
        };

        assert_eq!(a_book == b_book, false);
    }
}
