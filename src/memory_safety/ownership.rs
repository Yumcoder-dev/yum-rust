// When you create a new object in Rust, the assigned variable becomes the owner of the object.
//
// ownership provides a route to memory safety without needing a garbage collector
//
// There are two ways to shift ownership from one variable to another within a Rust program.
// 1) The first is by assignment.
// 2) The second is by passing data through a function barrier, either as an argument or a return value
//
// fn f1(arg){ <-----
//    // ...        |
// }                |  in this function call pass parameter means that arg = a; so arg is the new owner
// ...              |
// let a = ...;     |
// f1(a);    --------
//
// Four general strategies can help with ownership issues
// - Use references where full ownership is not required.
//     fn send(to: CubeSat, msg: Message) {                 fn send(to: &mut CubeSat, msg: Message) {
//         to.mailbox.messages.push(msg);           ---->    to.mailbox.messages.push(msg);
//     }                                                    }
// - Duplicate the value (Copy and Clone traits)
// - Refactor code to reduce the number of long-lived objects.
// - Wrap your data in a type designed to assist with movement issues (RC, ARC, RefCell)

mod test {
    #[allow(dead_code)]
    fn get_ownership_in_fn_call(input_v: Vec<i32>) -> usize {
        input_v.len()
    }

    #[test]
    fn def_ownership() {
        // when v goes out of scope, the Vec is dropped.
        // There can only be a single owner of an object at a time,
        // which ensures that only the owner drops it. This avoids double-free bugs.
        let v: Vec<i32> = Vec::new();
        //  If v is assigned to another variable, the ownership is transferred
        let v1 = v; //v1 is the new owner
        assert_eq!(v1.len(), 0)
        //v.len(); //error: Use of moved value
        // Since v1 is now the owner, access is no longer allowed through v
        // Although C++ too has move semantics, it doesn't prevent you from introducing a use-after-move bug.
    }

    #[test]
    fn def_ownership_with_fn() {
        let v: Vec<i32> = Vec::new();
        //  If v is assigned to another variable, the ownership is transferred
        get_ownership_in_fn_call(v); // ---> let input_v = v;
                                     // assert_eq!(v.len(), 0); //error: Use of moved value
                                     // value borrowed here after move
    }
}

// TODO example with fn
