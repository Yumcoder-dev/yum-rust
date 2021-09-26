pub mod arr_vec_slice;
pub mod str_string;

// This is not limited to slice-able or fat pointer types.
// In fact, you should always prefer using the borrowed type over borrowing
// the owned type. Such as
// &str over &String,
// &[T] over &Vec<T>,
// &T over &Box<T>.

// Using borrowed types you can avoid layers of indirection for those instances
// where the owned type already provides a layer of indirection.
// For instance, a String has a layer of indirection, so a &String will have two
// layers of indirection. We can avoid this by using &str instead, and letting
// &String coerce to a &str whenever the function is invoked.
