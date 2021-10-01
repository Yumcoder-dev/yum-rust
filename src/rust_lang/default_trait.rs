mod tests {
    use std::{path::PathBuf, time::Duration};
    // note that we can simply auto-derive Default here.
    #[derive(Default, Debug, PartialEq)]
    struct MyConfiguration {
        // Option defaults to None
        output: Option<PathBuf>,
        // Vecs default to empty vector
        search_path: Vec<PathBuf>,
        // Duration defaults to zero time
        timeout: Duration,
        // bool defaults to false
        check: bool,
    }

    impl MyConfiguration {
        // add setters here
    }

    #[test]
    fn test_default() {
        // construct a new instance with default values
        let mut conf = MyConfiguration::default();
        // do something with conf here
        conf.check = true;
        println!("conf = {:#?}", conf);

        // partial initialization with default values, creates the same instance
        let conf1 = MyConfiguration {
            check: true,
            ..Default::default()
        };
        assert_eq!(conf, conf1);
    }
}
