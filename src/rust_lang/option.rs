// Option can be viewed as a container that contains either zero or one element

mod test {
    #[test]
    fn test_into_iterator() {
        let x = Some("string");
        let v: Vec<&str> = x.into_iter().collect();
        assert_eq!(v, ["string"]);
        let x = None;
        let v: Vec<&str> = x.into_iter().collect();
        assert!(v.is_empty());
    }

    #[test]
    fn test_option() {
        let turing = Some("Turing");

        let mut logicians = vec!["Curry", "Kleene", "Markov"];
        logicians.extend(turing); // Since Option implements IntoIterator, it can be used as an argument to .extend()

        let mut logicians2 = vec!["Curry", "Kleene", "Markov"];
        // equivalent to
        if let Some(turing_inner) = turing {
            logicians2.push(turing_inner);
        }
        assert_eq!(logicians, logicians2)
    }

    #[test]
    fn test_chain() {
        //  If you need to tack an Option to the end of an existing iterator, you can pass it to .chain():
        let turing = Some("Turing");
        let logicians = vec!["Curry", "Kleene", "Markov"];

        let mut res = Vec::new();
        for logician in logicians.iter().chain(turing.iter()) {
            //println!("{} is a logician", logician);
            res.push(logician.clone());
        }
        assert_eq!(res, vec!["Curry", "Kleene", "Markov", "Turing"]);

        // println!("{:?} is a logician", logicians);
        assert_eq!(logicians, vec!["Curry", "Kleene", "Markov"])
    }
}
