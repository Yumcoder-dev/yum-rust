// Abuse the Deref trait to emulate inheritance between structs, and thus reuse methods.
// Sometimes we want to emulate the following common pattern from OO languages such as Java:
// class Foo {
//     void m() { ... }
// }
//
// class Bar extends Foo {}
//
// public static void main(String[] args) {
//     Bar b = new Bar();
//     b.m();
// }
mod tests {
    use std::ops::Deref;

    struct Foo {}

    impl Foo {
        #[allow(dead_code)]
        fn m(&self) {
            println!("call m!")
        }
    }

    struct Bar {
        f: Foo,
    }

    impl Deref for Bar {
        type Target = Foo;
        fn deref(&self) -> &Foo {
            &self.f
        }
    }

    // Advantages
    // You save a little boilerplate, e.g.,
    // impl Bar {
    //     fn m(&self) {
    //         self.f.m()
    //     }
    // }
    //
    // Disadvantages
    // this pattern interacts badly with bounds checking and thus generic programming
    // Finally, this pattern only supports single inheritance, and has no notion of
    // interfaces, class-based privacy, or other inheritance-related features. So,
    // it gives an experience that will be subtly surprising to programmers used to Java inheritance, etc.
    #[test]
    fn test_deref_polymorphism() {
        let b = Bar { f: Foo {} };
        b.m()
    }
}
