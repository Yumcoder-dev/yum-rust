mod test {
    #[allow(dead_code)]
    fn baz() {
        println!("in baz");
    }
    #[allow(dead_code)]
    fn bar() {
        // These don't need to be defined inside the function.
        struct Foo;

        // Implement a destructor for Foo.
        impl Drop for Foo {
            fn drop(&mut self) {
                println!("exit");
            }
        }

        // The dtor of _exit will run however the function `bar` is exited.
        let _exit = Foo;
        // Implicit return with `?` operator.
        baz();
    }

    #[test]
    fn test_finally() {
        bar();
    }
}
