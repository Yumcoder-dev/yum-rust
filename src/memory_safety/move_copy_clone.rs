// Move
// assigning one variable to another transfers the ownership to the assignee
//     1) let v:Vec<i32> = Vec::new();
//     2) let v1 = v;//v1 is the new owner
//
// in line 1:
//
//           Stack                    Heap
//      ----------------            -------
//     | buffer pointer |--------->|   1   |
//  v: |----------------|          |   2   |
//     |  capacity (4)  |          |   3   |
//     |      len (4)   |          |   4   |
//      ----------------            -------
//
// then in line 2:
//
//           Stack                    Heap
//      ----------------             -------
//     | buffer pointer |    |----->|   1   |
//  v: |----------------|    |      |   2   |
//     |  capacity (4)  |    |      |   3   |
//     |    len (4)     |    |      |   4   |
//      ----------------     |       -------
//     | buffer pointer |----|
// v1: |----------------|
//     |  capacity (4)  |
//     |    len (4)     |
//      ----------------
// What has essentially happened in the example is a shallow copy
// it is now v1's responsibility to drop the heap buffer and v can't touch it:
//      let v: Vec<i32> = Vec::new();
//      let v1 = v;
//      println!("v's length is {}", v.len()); //error: borrow of moved value: `v`
//
// Assignment is not the only operation which involves moves.
// 1) Values are also moved when passed as arguments or returned from functions:
//      let v: Vec<i32> = Vec::new();
//      // v is first moved into print_len's v1
//      // and then moved into v2 when print_len returns it
//      let v2 = print_len(v);
//      fn print_len(v1: Vec<i32>) -> Vec<i32> {
//          println!("v1's length is {}", v1.len());
//          v1 //v1 is moved out of the function
//      }
// 2) Or assigned to members of a struct or enum:
//      struct Numbers {
//          nums: Vec<i32>
//      }
//      let v: Vec<i32> = Vec::new();
//      //v moved into nums field of the Numbers struct
//      let n = Numbers { nums: v };
//
//      enum NothingOrString {
//          Nothing,
//          Str(String)
//      }
//      let s: String = "I am moving soon".to_string();
//      //s moved into the enum
//      let nos = NothingOrString::Str(s);
//

// Copies
// types which do not own other resources and can be bitwise copied are called Copy types.
// They implement the Copy marker trait. All primitive types like integers, floats and characters are Copy.
// Structs or enums are not Copy by default but you can derive the Copy trait
//      #[derive(Copy, Clone)]
//      struct Point {
//          x: i32,
//          y: i32,
//      }
//
//      #[derive(Copy, Clone)]
//      enum SignedOrUnsignedInt {
//          Signed(i32),
//          Unsigned(u32),
//      }
//
// For #[derive(Copy, Clone)] to work, all the members of the struct or enum must be Copy themselves.
// For example, this will not work:
//      error:the trait `Copy` may not be implemented for this type
//      because its nums field does not implement `Copy` (Vec implements Clone, but not Copy)
//
//      #[derive(Copy, Clone)]
//      struct Numbers {
//          nums: Vec<i32>
//      }
//
// Clone
// When a value is moved, Rust does a shallow copy; but what if you want to create a deep copy like in C++?
// To allow that, a type must first implement the Clone trait. Then to make a deep copy, client
// code should call the clone method

#[cfg(test)]
mod tests {

    // Copy is implicit, inexpensive, and cannot be re-implemented (memcpy).
    // Clone is explicit, may be expensive, and may be re-implement arbitrarily.
    //
    // Differs from Copy in that Copy is implicit and extremely inexpensive, while Clone is always explicit
    // and may or may not be expensive. In order to enforce these characteristics, Rust does not allow you
    // to reimplement Copy, but you may reimplement Clone and run arbitrary code
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PointCloneAndCopy {
        pub x: f64,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct PointCloneOnly {
        pub x: f64,
    }
    #[test]
    fn test_copy() {
        let p1 = PointCloneAndCopy { x: 0. };
        let p2 = p1; // because type has `Copy`, it gets copied automatically.
        println!("{:p} {:p}", &p1, &p2);
        assert_eq!(p1, p2);
    }
    #[test]
    fn test_clone_only() {
        let p1 = PointCloneOnly { x: 0. };
        // let p2 = p1; // because type has no `Copy`, this is a move instead.
        let p2 = p1.clone();
        println!("{:p} {:p}", &p1, &p2);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_clone_vec() {
        //
        //           Stack                    Heap
        //      ----------------            -------
        //     | buffer pointer |--------->|   1   |
        //  v: |----------------|          |   2   |
        //     |  capacity (4)  |          |   3   |
        //     |    len (4)     |          |   4   |
        //      ----------------            -------
        //     | buffer pointer | ----
        // v1: |----------------|     |     -------
        //     |  capacity (4)  |      --> |   1   |
        //     |    len (4)     |          |   2   |
        //      ----------------           |   3   |
        //                                 |   4   |
        //                                  -------
        //
        // Vec implements Clone, but not Copy
        let v = vec![10, 20, 30, 40, 50];
        let v1 = v.clone(); // ok since Vec implements Clone
        println!("v: {:p}, v1:{:p}", &v, &v1);
        assert_eq!(v, v1);
    }
}
