// Borrowing introduces aliasing.
// A reference can be borrowed from the owner

// To summarize, the usage rules for immutable and mutable references are as follows.
// For a value, there can be:
//   Either many immutable references.
//   Or one mutable reference.
// But not both at the same time.
//
// very similar to https://en.wikipedia.org/wiki/Readers–writer_lock
//
// another term for immutable references is shared references

#[cfg(test)]
mod tests {
    #[test]
    fn test_def_references() {
        let x: i32 = 42;
        // To create a reference to x, you'd use the & operator:
        let r = &x;
        // read the value of r
        let v = *r;
        assert_eq!(v, x);
        // All the values and references created above were immutable,
        // which is the default in Rust. If you want to change the value through
        // a reference, create a mutable reference.
    }

    #[test]
    fn test_def_mut_references() {
        // The &mut operator creates a mutable reference
        // let x: i32 = 42;
        // let m = &mut x; // cannot borrow `x` as mutable, as it is not declared as mutable

        let mut x: i32 = 42;
        let m = &mut x;
        *m = 100;
        assert_eq!(x, 100);
    }

    #[test]
    fn def_borrowing_with_multiple_shared_references() {
        let v: Vec<i32> = Vec::new();
        // multiple shared references
        let v1 = &v; //v1 has borrowed from v
        let v2 = &v; //v2 has also borrowed from v
                     // v.len(); //allowed
                     // v1.len(); //also allowed
                     // v2.len(); //also allowed

        // This is safe since there is no mutation.
        assert_eq!(v.len(), v1.len());
        assert_eq!(v.len(), v2.len());
    }

    #[test]
    fn test_borrow_box() {
        let x1 = 42;
        let y1 = Box::new(84);
        {
            // starts a new scope
            let _z = (x1, y1);
            // z goes out of scope, and is dropped;
            // it in turn drops the values from x1 and y1
        }
        // x1's value is Copy, so it was not moved into z
        let x2 = x1;
        // y1's value is not Copy, so it was moved into z
        // let y2 = y1;

        assert_eq!(x2, x1)
    }
}

// pub fn borrowing_prevent_use_after_free_bug() {
//     // a borrower cannot access the resource after the owner has destroyed it
//     // because otherwise it will lead to a use-after-free bug:
//     let v1: &Vec<i32>;
//     {
//         let v = Vec::new();
//         v1 = &v;
//     } //v is dropped here
//     v1.len(); //error:borrowed value does not live long enough
// }

// If the lifetime of a reference is smaller than the lifetime of the referent,
// the code compiles, otherwise it doesn't.
// pub fn How_rust_disallows_both_aliasing_and_mutation_at_the_same_time() {
//     let v1: &Vec<i32>; //-------------------------+
//     {
//         //                                         |
//         let v = Vec::new(); //-----+               |v1's lifetime
//         v1 = &v; //                | v's lifetime  |
//     } //<--------------------------+               |
//     v1.len(); //<---------------------------------+
// }

// Although there can be multiple shared references,
// there can only be one mutable reference at one time:
// pub fn prevent_borrowing_multiple_mut() {
//     let mut v: Vec<i32> = Vec::new();
//     // We cannot have two mutable references to a value at the same time.
//     let v1 = &mut v; //first mutable reference
//     let v2 = &mut v; //second mutable reference
//     v1.push(1); //error:cannot borrow `v` as mutable more than once at a time
// }

// If Rust had allowed a mutable reference and an immutable reference at the same time,
// the memory could become invalid through the mutable reference while the immutable
// reference could still be pointing to that invalid memory. For example, in the code below,
// v1 could access invalid memory if such code was allowed:
// pub fn why_mutable_and_immutable_reference_disallow_at_the_same_time() {
//     let mut v = vec![0, 1, 2, 3];
//     let v1 = &v[0]; //an immutable reference to Vec's first element
//     v.push(4); //this can invalidate Vec's internal buffer
//     let v2 = *v1; //this could access invalid memory live long enough

//     // see ../use_borrowed_args/arr_vec_slice.rs
//     // function --> heap_vec_dynamic_alloc
//     // when push in v, it may be reallocated v in memory!
// }
