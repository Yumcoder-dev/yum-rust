mod test {
    #[allow(unused)]
    use std::mem::{size_of, size_of_val};

    #[allow(dead_code)]
    fn print_array_size(arr: [i32; 5]) -> usize {
        // println!(
        //     "Array size in print_array_size function: {}",
        //     size_of_val(&arr)
        // );
        size_of_val(&arr)
    }

    #[test]
    fn def_arr() {
        // - In Rust, an array's size is part of the type.
        //   For example, this code will not compile:
        //   let array: [i32; 4] = [0, 1, 2];
        // - Access to elements in Rust does bounds checking
        // - creating in Stack
        let m_array = [10, 20, 30, 40, 50];
        // println!("array {:?}", m_array);
        assert_eq!(print_array_size(m_array), 20); // only array of size 5 is allow to pass through the function
    }

    #[test]
    pub fn def_vec() {
        // vector
        // The big limitation of arrays is that they are fixed in size.
        // In contrast, vectors can grow at runtime
        //
        // create in heap
        // point to heap + len + capacity
        //
        //
        //                Stack                    Heap
        //           ----------------            -------
        //          | buffer pointer |--------->|   1   |
        //  m_vec:  |----------------|          |   2   |
        //          |  capacity (4)  |          |   3   |
        //          |      len (4)   |          |   4   |
        //           ----------------            -------
        let mut m_vec = vec![1, 2, 3];
        //println!("v has {} elements", m_vec.len());
        assert_eq!(m_vec.len(), 3);
        //but you can add more at runtime
        m_vec.push(4);
        m_vec.push(5);
        // println!("m_vec has {} elements", m_vec.len());
        assert_eq!(m_vec.len(), 5);
    }

    #[test]
    pub fn heap_vec_dynamic_alloc() {
        // How does a vector allow dynamic growth?
        // Internally, a vector keeps all the elements in an array allocated on the heap.
        // When a new element is pushed, the vector checks if there is still some capacity
        // left in the array. If not, the vector allocates a bigger array,
        // copies all the elements to the new array and deallocates the old array.
        //
        // dynamic allocation in Vec
        //                Stack                    Heap               Heap
        //           ----------------            -------            -------
        //          | buffer pointer |----      |   1   |      --->|   1   |
        //  m_vec:  |----------------|   |      |   2   |     |    |   2   |
        //          |  capacity (4)  |   |      |   3   |     |    |   3   |
        //          |      len (4)   |   |      |   4   |     |    |   4   |
        //           ----------------    |       -------      |    |   5   |
        //                                --------------------     |   5   |
        //                                                         |  ...  |
        //                                                          -------
        //                                        old memory     new allocated memory
        let mut m_vec: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(m_vec.capacity(), 4);
        // println!("Address of v's first element: {:p}", &m_vec[0]); //{:p} prints the address

        for _ in 1..101 {
            // force to simulate real app behavior (and force realloc)
            m_vec.push(5);
        }

        assert_eq!(128, m_vec.capacity());
        // println!("Address of v's first element: {:p}", &m_vec[0]);

        // If you do not see a different address after pushing more elements onto a vector,
        // it might be because the allocator had enough space at the end of the original
        // buffer such that the new and the old buffers have the same starting address.
        // Try pushing more elements and you will see a different address.
        // Read about C library function realloc to understand how this might happen.
    }

    #[test]
    pub fn def_slice() {
        // Slices act like temporary views into an array or a vector.
        // Since slices can be created from both arrays and vectors,
        // they are a very powerful abstraction. Hence for arguments
        // in functions, the default choice should be to accept
        // a slice instead of an array or a vector. In fact many functions like len,
        // is_empty etc. work on slices instead of on vectors or arrays.
        //
        //
        //                Stack                     Heap
        //           ----------------            --------
        //          | buffer pointer |--------->|   10   |
        //  arr:    |----------------|    ----->|   20   |
        //          |  capacity (4)  |   |      |   30   |
        //          |      len (4)   |   |      |   40   |
        //           ----------------    |       --------
        //                               |
        //           -----------------   |
        //  s:      |  buffer pointer |--
        //          |      len (2)    |
        //           -----------------
        //
        let arr: [i32; 4] = [10, 20, 30, 40];
        // may be give it as arg
        // in addition to a pointer to the second element in v's buffer,
        // s also has an 8 byte length field with value 2
        let s = &arr[1..3];
       
        // The [1..3] syntax creates a range from index 1 (inclusive) to 3 (exclusive).
        // If you omit the first number in the range ([..3]) it defaults to zero and
        // if you omit the last number ([1..]) it defaults to the length of the array.
        // If you print the elements in the [1..3] slice, you get 20 and 30:
        assert_eq!(20, s[0]);
        assert_eq!(30, s[1]);

        // println!("Third element in slice: {:}", s[2]); //panics: index out of bounds
    }

    #[test]
    fn fat_pointer_size() {
        // The term fat pointer refers to memory layout. Thin pointers, such as
        // raw pointers, are a single usize wide. Fat pointers are usually two usize wide,
        // and occasionally more.
        assert_eq!(8, size_of::<&i32>());
        assert_eq!(16, size_of::<&[i32]>()); // 8 + 8 (buffer pointer + len)
    }

    #[test]
    pub fn borrow_in_slice() {
        let v: Vec<i32> = vec![1, 2, 3, 4];
        let s = &v[..];
        // v.push(5); // error

        // Why? Because when the slice is created, it points to the first element of
        // the vector's backing buffer and as a new element is pushed onto the vector,
        // it allocates a new buffer and the old buffer is deallocated.
        // This leaves the slice pointing to an invalid memory address,
        // which if accessed would have lead to undefined behaviour.
        // Rust has saved you from disaster again.
        assert_eq!(1, s[0]);
    }

    // TODO write example for performance of reserve with benchmark
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve
}
