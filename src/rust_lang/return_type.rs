mod test {
    use std::fmt::Debug;
    #[test]
    fn test_return_type() {
        #[allow(dead_code)]
        fn report<T: Debug>(item: T) {
            println!("{:?}", item);
        }
        #[allow(dead_code)]
        fn clear(text: &mut String) -> () {
            *text = String::from("");
        }
        #[allow(dead_code)]
        // The exclamation symbol, !, is known as the “Never” type. Never indicates that a
        // function never returns, especially when it is guaranteed to crash
        fn dead_end() -> ! {
            panic!("you have reached a dead end");
        }
        #[allow(dead_code)]
        fn forever() -> ! {
            let mut cnt = 0;
            loop {
                //...
                cnt += 1;
                if cnt > 5 {
                    // break; uncomment return err, note to return type
                }
            }
        }

        forever();
    }
}
