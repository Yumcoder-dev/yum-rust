// uncomment the following bench
// #![feature(test)]

#![feature(log_syntax)]

#[macro_use]
extern crate crossbeam;

mod anti_pattern;
mod concurrency;
mod design_pattern;
mod memory_safety;
mod rust_lang;
mod use_borrowed_args;
// recommended rust project structure
// https://doc.rust-lang.org/cargo/guide/project-layout.html

fn main() {
    println!("rust idioms");
}
