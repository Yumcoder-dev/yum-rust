mod test {
    use std::collections::HashMap;

    type Data = HashMap<String, u32>;

    trait Formatter {
        fn format(&self, data: &Data, buf: &mut String);
    }

    struct Report;

    impl Report {
        // Write should be used but we kept it as String to ignore error handling
        fn generate<T: Formatter>(g: T, s: &mut String) {
            // backend operations...
            let mut data = HashMap::new();
            data.insert("one".to_string(), 1);
            data.insert("two".to_string(), 2);
            // generate report
            g.format(&data, s);
        }
    }

    struct Text;
    impl Formatter for Text {
        fn format(&self, data: &Data, buf: &mut String) {
            for (k, v) in data {
                let entry = format!("{} {}\n", k, v);
                buf.push_str(&entry);
            }
        }
    }

    struct Json;
    impl Formatter for Json {
        fn format(&self, data: &Data, buf: &mut String) {
            buf.push('[');
            for (k, v) in data.into_iter() {
                let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
                buf.push_str(&entry);
                buf.push(',');
            }
            buf.pop(); // remove extra , at the end
            buf.push(']');
        }
    }

    #[test]
    fn test_strategy() {
        let mut s = String::from("");
        Report::generate(Text, &mut s);
        assert!(s.contains("one 1"));
        assert!(s.contains("two 2"));

        s.clear(); // reuse the same buffer
        Report::generate(Json, &mut s);
        assert!(s.contains(r#"{"one":"1"}"#));
        assert!(s.contains(r#"{"two":"2"}"#));
    }
}
