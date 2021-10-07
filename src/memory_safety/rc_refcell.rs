// Rc<T> is not thread-safe. In multithreaded code, it’s much better to
// replace Rc<T> with Arc<T> and Rc<RefCell<T>> with Arc<Mutex<T>>. Arc
// stands for atomic reference counter.
//
// With interior mutability, you may want to provide an argument to a method that
// takes immutable values, yet you need to retain mutability. If you’re willing to pay the
// runtime performance cost, it’s possible to fake immutability. If the method requires
// an owned value, wrap the argument in Cell<T>. References can also be wrapped in
// RefCell<T>. It is common when using the reference counted types Rc<T> and Arc<T>,
// which only accept immutable arguments, to also wrap those in Cell<T> or Ref-
// Cell<T>. The resulting type might look like Rc<RefCell<T>>. This means that you pay
// the runtime cost twice but with significantly more flexibility.
