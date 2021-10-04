mod tests {
    #[test]
    fn test_how_fast_your_computer() {
        use std::time::{Duration, Instant};
        let mut count = 0;
        let time_limit = Duration::new(1, 0);
        let start = Instant::now();
        while (Instant::now() - start) < time_limit {
            count += 1;
        }
        println!("{}", count);
    }

    #[test]
    fn test_loop_until() {
        let mut samples = vec![];
        let mut count = 0;
        while samples.len() < 10 {
            // let sample = take_sample();
            // if is_outlier(sample) {
            //     continue;
            // }
            count += 1;
            samples.push(count);
        }
        assert_eq!(count, 10)
    }

    #[test]
    fn test_abort_loop() {
        // ctrl + click and see zip application
        for (x, y) in (0..).zip(0..) {
            println!("{} {}", x, y);
            if x + y > 100 {
                break;
            }
        }
    }

    #[test]
    fn test_break_return() {
        let n = loop {
            break 123;
        };
        assert_eq!(123, n);
    }

    #[test]
    fn test_let_match() {
        fn is_even(n: i32) -> bool {
            n % 2 == 0
        }

        let n = 654321;
        let description = match is_even(n) {
            true => "even",
            false => "odd",
        };
        println!("{} is {}", n, description);

        let n = 123456;
        let description = if is_even(n) { "even" } else { "odd" };
        println!("{} is {}", n, description);
    }

    #[test]
    fn test_iter() {
        // let container = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut container = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for item in container {
            // access Ownership
            println!("{}", item);
        }

        // for item in container {
        //     // access Ownership
        //     println!("{}", item);
        // }

        // for item in IntoIterator::into_iter(container) {
        //     println!("{}", item);
        // }

        // for item in container.into_iter() {
        //     println!("{}", item);
        // }

        // for item in &container {
        //     println!("{}", item);
        // }

        // for item in container.iter() {
        //     println!("{}", item);
        // }

        // for item in &mut container {
        //     *item += 2;
        //     println!("{}", item);
        // }
        // for item in &mut container {
        //     *item += 2;
        //     println!("{}", item);
        // }
    }
}
