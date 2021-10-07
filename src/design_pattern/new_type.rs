mod test {
    #[test]
    fn test_new_type() {
        type File = String;
        // type alias accept both String and File type
        fn connect_alias(host: File) {
            println!("{}", host);
        }
        let my_file: File = "my file path".to_string();
        connect_alias(my_file);
        connect_alias("string!".to_string());

        // --------- pattern --------
        struct Hostname(String); // expected-Hostname is our new type.
        fn connect(host: Hostname) {
            println!("connected to {}", host.0);
        }

        let ordinary_string = String::from("localhost");
        let host = Hostname(ordinary_string.clone());
        connect(host);
        // connect(ordinary_string); // app raise err
    }
}
