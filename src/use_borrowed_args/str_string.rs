// Why rust has two string types: String and &str
//  a string literal is of type &str and
//  does not appear compatible with the type String
pub fn print_me_string(msg: String) {
    println!("{}", msg);
}

// let message = "hello world".to_string(); // analogous to using clone()
// print_str(message);
//
// &str
// The & symbol is a reference type and means we are borrowing the variable.
//
// Using reference is more efficent. Using String for message
// means the program must copy the value. When using a reference
// such as &str, no copy is made.
//
// String
// A String type can be magically turned into a &str and
// type using Deref trait and type corecion.
//
//
// let str = "hello world";
// print_me_str(str);
//
// let owned_string = "hello world".to_string(); // or String::from_str("hello world")
// print_me_str(&owned_string);
//
// let counted_string = std::rc::Rc::new("hello world".to_string());
// print_me_str(&counted_string);
//
// let atomically_counted_string = std::sync::Arc::new("hello world".to_string());
// print_me_str(&atomically_counted_string);

#[allow(dead_code)]
pub fn print_me_str(msg: &str) {
    println!("{}", msg);
}

//////////////////////////////////////////////////////////
// struct Person {
//     name: String,
// }

// impl Person {
//     fn greet(&self) {
//         println!("Hello, my name is {}", self.name);
//     }
// }

// fn main() {
//     let name = String::from_str("Herman");
//     let person = Person { name: name };
//     person.greet();
//     println!("My name is {}", name); // move error
// }
//////////////////////////////////////////////////////////
struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

pub fn str_in_struct() {
    let name = "str_in_struct";
    let person_manually_created = Person { name: name };
    let preson_new = Person::new(name);

    person_manually_created.greet();
    preson_new.greet();

    println!("My name is {}", name); // see above example
}
