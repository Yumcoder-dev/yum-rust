// Trait objects add a form of polymorphism—the ability to share an interface between
// types—to Rust via dynamic dispatch. Trait objects are similar to generic objects. Generics
// offer polymorphism via static dispatch. Choosing between generics and type objects
// typically involves a trade off between disk space and time:
//  - Generics use more disk space with faster run times.
//  - Trait objects use less disk space but incur a small runtime overhead caused by pointer indirection.
// Trait objects are dynamically-sized types, which means that these are always seen in the
// wild behind a pointer. Trait objects appear in three forms: &dyn Trait, &mut dyn Trait,
// and Box<dyn Trait>.
// 1-> The primary difference between the three forms is that
// Box<dyn Trait> is an owned trait object, whereas the other two are borrowed.
//
// why dyn is really needed?
// use rand::Rng;
// use rand::rngs::ThreadRng;
// Although these both have something to do with random number generators, they’re
// quite different. rand::Rng is a trait; rand::rngs::ThreadRng is a struct. Trait objects
// make this distinction harder.
// Before the Rust 2018 edition, the situation was even more confusing. The dyn keyword
// did not exist. This meant that context was needed to decide between &Rng and
// &ThreadRng.

// see ./design_pattern/command_bench.rs
