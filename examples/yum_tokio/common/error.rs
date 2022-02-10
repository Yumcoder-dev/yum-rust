#[derive(Debug)]
pub enum Error {
    /// Not enough data is available to parse a message
    Incomplete,
    Other(crate::Error),
}

mod test {
    use crate::common::error::Error;
    use core::fmt;

    #[test]
    fn test_using_error() {
        #[derive(Debug, Clone)]
        struct MissingFirstElementInArrayErr;
        impl fmt::Display for MissingFirstElementInArrayErr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "first item does not exist!")
            }
        }

        #[derive(Debug, Clone)]
        struct InvalidElementInArrayErr;
        impl fmt::Display for InvalidElementInArrayErr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "invalid item in array!")
            }
        }

        type MyError = Box<dyn std::error::Error + Send + Sync>;
        type MyResult<T> = std::result::Result<T, MyError>;

        // fn getFistElement(vec: Vec<&str>) -> Result<i32, std::error::Error> { // the size for values of type `(dyn std::error::Error + 'static)` cannot be known at compilation time
        fn getFistElement(vec: Vec<&str>) -> MyResult<i32> {
            vec.first()
                .ok_or(MissingFirstElementInArrayErr)
                .and_then(|item| {
                    item.parse::<i32>()
                        .map_err(InvalidElementInArrayErr)
                        .map(|i| i * 2)
                });
        }
        // println!("{:?}", Error::Incomplete);

        let numbers = vec!["42", "93", "18"];
        assert_eq!(getFistElement(numbers), 42 * 2);
        // let empty = vec![];
        // let strings = vec!["tofu", "93", "18"];
    }
}
