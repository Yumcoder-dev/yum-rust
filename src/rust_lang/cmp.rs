mod tests {
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
        assert_eq!(a_book == BookFormat::HardBack, true);
        assert_eq!(BookFormat::HardBack == a_book, true);

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
