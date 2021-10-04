// approaches:
// 1. Using trait objects
// 2. Using function pointers
// 3. Using Fn trait objects
//
// If our commands are small and may be defined as functions or passed as a
// closure then using function pointers might be preferable since it does
// not exploit dynamic dispatch. But if our command is a whole struct with
// a bunch of functions and variables defined as separated module then
// using trait objects would be more suitable.
// Static dispatch gives faster performance, while dynamic dispatch provides
// flexibility when we structure our application.
// see command_bench.rs
mod test {
    // note: dyn is a prefix of a trait object’s type.
    // An example of a trait object:
    // trait Printable {
    //     fn stringify(&self) -> String;
    // }
    // impl Printable for i32 {
    //     fn stringify(&self) -> String {
    //         self.to_string()
    //     }
    // }
    // fn print(a: Box<dyn Printable>) {
    //     println!("{}", a.stringify());
    // }
    // fn main() {
    //     print(Box::new(10) as Box<dyn Printable>);
    // }

    #[test]
    fn test_command_using_trait_object() {
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

    #[test]
    fn test_using_function_pointers() {
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

    // TODO add example for using: Fn, FnMut, and FnOnce
    // also see: https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/
    #[test]
    fn test_using_fn_trait_objects() {
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
}
