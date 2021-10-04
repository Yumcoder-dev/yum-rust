mod test {
    #[test]
    fn test_imperative() {
        let mut sum = 0;
        for i in 1..11 {
            sum += i;
        }
        assert_eq!(55, sum);
    }

    #[test]
    fn test_declarative() {
        assert_eq!(55, (1..11).fold(0, |a, b| a + b));
    }
}
