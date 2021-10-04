mod test {
    #[test]
    fn test_temporary_mutability() {
        // Compiler ensures that you don't accidentally mutate data after some point.
        let data = {
            let mut data = vec!["3", "2", "1"];
            data.sort();
            // println!("--{:?}", data);
            data
        };
        assert_eq!(data, vec!["1", "2", "3"]);
    }
}
