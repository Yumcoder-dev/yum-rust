/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}
/// Creates a new, empty `File`.
///
/// # Examples
///
/// ```
/// let f = File::new("f1.txt");
/// ```

// cargo doc --open
