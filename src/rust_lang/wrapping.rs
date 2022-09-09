mod test {
    #[test]
    fn test_wrapping() {
        use std::num::Wrapping;

        let index = Wrapping(2u8);
        let offset = Wrapping(5u8);

        let res = index - offset;
        assert_eq!(253, res.0);
    }
}
