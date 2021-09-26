// Did you know that around 70% serious security bugs
// in Chrome and Microsoft's products are memory safety issues
//
// - Manual memory management:
//   In C the programmer is responsible for managing the memory
//   If the programmer has allocated a chunk of memory in their
//   program using malloc, it is their responsibility to release
//   that chunk by calling free exactly once
// - Garbage collection
//   In garbage collected languages like Java and C#, the programmer
//   is free to allocate memory but has no obligation to free it.
//   The garbage collector keeps track of allocated memory that is
//   no longer used and periodically reclaims it. While it works in
//   providing memory safety, it comes at a performance cost due to
//   the garbage collector running in the background.
// - RAII stands for Resource Acquisition Is Initialization
//   The idea is much simpler — when a variable in a program is no longer used,
//   free the memory owned by that variable. If the compiler can do this analysis,
//   then it can insert the appropriate code to free the memory. This idiom first
//   originated in C++
//
//
// To safely free an object, there must be no references to it,
// otherwise you'll end up with a dangling pointer.
// Similarly, if a thread wants to send an object to another thread,
// there can't be a reference to it on the sending thread.
// There are two elements in play here: aliasing and mutation.
// If the object was not being destroyed or sent across a thread,
// there is nothing wrong with having references to it.
// It is only when both of them are combined that you get in trouble.

// In light of this observation, Rust's solution to memory safety is to simply
// disallow both aliasing and mutation at the same time,
// and Rust achieves this through ownership and borrowing.
pub mod borrowing;
pub mod ownership;
