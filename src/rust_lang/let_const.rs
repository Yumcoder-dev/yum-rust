mod test {
    #[test]
    fn test_let_vs_constant() {
        let uname = "Yumcoder";
        // print!("{:p}", &uname);
        let uname = uname.len();
        // print!("{:p}", &uname);
        assert_eq!(8, uname);

        const _NAME: &str = "Yumcoder";
        // const NAME: usize = NAME.len();
    }
}
