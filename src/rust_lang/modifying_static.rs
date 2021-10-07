// Accessing and modifying static mut variables requires the use of an unsafe block.
// This is Rust’s way of disclaiming all responsibility.

// Mutable global variables are denoted with static mut.
// By convention, global variables use ALL CAPS.
// A const keyword is included for values that never change.
static mut ERROR: i32 = 0;
mod test {
    #[test]
    fn test_modify_static() {
        // let mut f = std::fs::File::new("something.txt");
        // read(f, buffer);
        unsafe {
            if super::ERROR != 0 {
                panic!("An error has occurred while reading the file ")
            }
        }
    }
}
