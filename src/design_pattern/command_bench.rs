// rust-analyzer doest not recognized external crate
// extern crate test;

// #[allow(unused)]
// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     #[test]
//     fn it_works() {
//         assert_eq!(4, add_two(2));
//     }

//     #[bench]
//     fn bench_add_two(b: &mut Bencher) {
//         b.iter(|| add_two(2));
//     }
// }

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn command_using_trait_object() {
    pub trait Migration {
        fn execute(&self) -> &str;
        fn rollback(&self) -> &str;
    }

    pub struct CreateTable;
    impl Migration for CreateTable {
        fn execute(&self) -> &str {
            "create table"
        }
        fn rollback(&self) -> &str {
            "drop table"
        }
    }

    pub struct AddField;
    impl Migration for AddField {
        fn execute(&self) -> &str {
            "add field"
        }
        fn rollback(&self) -> &str {
            "remove field"
        }
    }

    struct Schema {
        commands: Vec<Box<dyn Migration>>,
    }

    impl Schema {
        fn new() -> Self {
            Self { commands: vec![] }
        }

        fn add_migration(&mut self, cmd: Box<dyn Migration>) {
            self.commands.push(cmd);
        }

        fn execute(&self) -> Vec<&str> {
            self.commands.iter().map(|cmd| cmd.execute()).collect()
        }
        fn rollback(&self) -> Vec<&str> {
            self.commands
                .iter()
                .rev() // reverse iterator's direction
                .map(|cmd| cmd.rollback())
                .collect()
        }
    }

    let mut schema = Schema::new();

    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}

fn using_function_pointers() {
    type FnPtr = fn() -> String;
    struct Command {
        execute: FnPtr,
        rollback: FnPtr,
    }

    struct Schema {
        commands: Vec<Command>,
    }

    impl Schema {
        fn new() -> Self {
            Self { commands: vec![] }
        }
        fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
            self.commands.push(Command { execute, rollback });
        }
        fn execute(&self) -> Vec<String> {
            self.commands.iter().map(|cmd| (cmd.execute)()).collect() // note () around fn pointer
        }
        fn rollback(&self) -> Vec<String> {
            self.commands
                .iter()
                .rev()
                .map(|cmd| (cmd.rollback)()) // note () around fn pointer
                .collect()
        }
    }

    fn add_field() -> String {
        "add field".to_string()
    }

    fn remove_field() -> String {
        "remove field".to_string()
    }

    let mut schema = Schema::new();
    // Since function pointers implement all three traits Fn, FnMut, and FnOnce
    // we could as well pass and store closures instead of function pointers.
    schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
    schema.add_migration(add_field, remove_field);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}

fn using_fn_trait_objects() {
    type Migration<'a> = Box<dyn Fn() -> &'a str>;

    struct Schema<'a> {
        executes: Vec<Migration<'a>>,
        rollbacks: Vec<Migration<'a>>,
    }

    impl<'a> Schema<'a> {
        fn new() -> Self {
            Self {
                executes: vec![],
                rollbacks: vec![],
            }
        }
        fn add_migration<E, R>(&mut self, execute: E, rollback: R)
        where
            E: Fn() -> &'a str + 'static,
            R: Fn() -> &'a str + 'static,
        {
            self.executes.push(Box::new(execute));
            self.rollbacks.push(Box::new(rollback));
        }
        fn execute(&self) -> Vec<&str> {
            self.executes.iter().map(|cmd| cmd()).collect()
        }
        fn rollback(&self) -> Vec<&str> {
            self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
        }
    }

    fn add_field() -> &'static str {
        "add field"
    }

    fn remove_field() -> &'static str {
        "remove field"
    }

    let mut schema = Schema::new();
    schema.add_migration(|| "create table", || "drop table");
    schema.add_migration(add_field, remove_field);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}

#[allow(dead_code)]
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    group.bench_function("command_using_trait_object", |b| {
        b.iter(|| command_using_trait_object())
    });
    group.bench_function("using_function_pointers", |b| {
        b.iter(|| using_function_pointers())
    });
    group.bench_function("using_fn_trait_objects", |b| {
        b.iter(|| using_fn_trait_objects())
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
