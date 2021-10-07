// Why rust has two string types: String and &str
//  a string literal is of type &str and
//  does not appear compatible with the type String
//
// &str
// The & symbol is a reference type and means we are borrowing the variable.
//
// String
// A String type can be magically turned into a &str and
// type using Deref trait and type corecion.
//
// Using reference is more efficient. Using String for message
// means the program must copy the value. When using a reference
// such as &str, no copy is made.
//////////////////////////////////////////////////////////
mod test {
    #[allow(unused_imports)]
    use regex::Regex;
    #[allow(unused_imports)]
    use std::str::FromStr;

    #[allow(dead_code)]
    fn print_me_str(msg: &str) {
        println!("{}", msg);
    }
    #[allow(dead_code)]
    fn print_me_string(msg: String) {
        println!("{}", msg);
    }

    #[test]
    fn test_print_me_str() {
        let message = "hello world".to_string();
        print_me_str(&*message);

        let str = "hello world";
        print_me_str(str);
        let owned_string = "hello world".to_string(); // or String::from_str("hello world")
        print_me_str(&owned_string);

        let counted_string = std::rc::Rc::new("hello world".to_string());
        print_me_str(&counted_string);

        let atomically_counted_string = std::sync::Arc::new("hello world".to_string());
        print_me_str(&atomically_counted_string);
    }
    #[allow(dead_code)]
    fn test_print_me_string() {
        let message = "hello world".to_string();
        print_me_string(message);

        let str = "hello world";
        // print_me_string(str); // error
        print_me_string(str.to_string());
    }
    #[allow(dead_code)]
    struct Person<'a> {
        name: &'a str,
    }

    impl<'a> Person<'a> {
        #[allow(dead_code)]
        fn new(name: &'a str) -> Self {
            Self { name }
        }
        #[allow(dead_code)]
        fn greet(&self) {
            println!("Hello, my name is {}", self.name);
        }
    }

    #[test]
    fn test_string_pass_to_struct() {
        let from_str_name = String::from_str("Yumcoder").unwrap();
        // let owned_str_name = "Yumcoder".to_owned();
        let person = Person {
            name: &from_str_name,
        };
        person.greet();
        // struct Person {
        //     name: String,
        // }
        // impl Person {
        //     fn greet(&self) {
        //         println!("Hello, my name is {}", self.name);
        //     }
        // }
        println!("My name is {}", from_str_name); // if Person.name is String, this line throw move error
        assert_eq!(from_str_name, "Yumcoder");
    }

    #[test]
    fn test_str_in_struct() {
        let name = "str_in_struct";
        let person_manually_created = Person { name };
        let person_new = Person::new(name);

        person_manually_created.greet();
        person_new.greet();

        // println!("My name is {}", name); // see above example
        assert_eq!(name, "str_in_struct");
    }

    #[test]
    fn test_string_lines_with_line_num() {
        let search_term = "picture";
        let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

        let mut line_num: usize = 1;
        for line in quote.lines() {
            if line.contains(search_term) {
                println!("{}: {}", line_num, line);
            }
            line_num += 1;
        }
    }

    #[test]
    fn test_string_lines_with_line_num_more_economic() {
        let search_term = "picture";
        let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";
        for (i, line) in quote.lines().enumerate() {
            if line.contains(search_term) {
                let line_num = i + 1;
                println!("{}: {}", line_num, line);
            }
        }
    }

    #[test]
    fn test_string_lines_with_regex() {
        let re = Regex::new("picture").unwrap();
        let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";
        for line in quote.lines() {
            let contains_substring = re.find(line);
            match contains_substring {
                Some(_) => println!("{}", line),
                None => (),
            }
        }
    }
}
