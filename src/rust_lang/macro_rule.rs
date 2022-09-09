// https://danielkeep.github.io/tlborm/book/README.html

mod test {
    // There can be multiple branches in a single macro expanding to different
    // code based on different arguments. Each branch can take multiple arguments,
    // starting with the $ sign and followed by a token type:

    // item — an item, like a function, struct, module, etc.
    // block — a block (i.e. a block of statements and/or an expression, surrounded by braces)
    // stmt — a statement
    // pat — a pattern
    // expr — an expression
    // ty — a type
    // ident — an identifier
    // path — a path (e.g., foo, ::std::mem::replace, transmute::<_, int>, …)
    // meta — a meta item; the things that go inside #[...] and #![...] attributes
    // tt — a single token tree
    // vis — a possibly empty Visibility qualifier

    #[test]
    fn test_macro_rules_four() {
        macro_rules! four {
            // This matches if and only if the input is also empty
            // (i.e. four!(), four![] or four!{}).
            () => {
                1 + 3
            };
        }

        assert_eq!(four!(), 4);
        assert_eq!(four! {}, 4);
        assert_eq!(four![], 4);
    }

    #[test]
    fn test_macro_rules_add() {
        macro_rules! add {
            // first arm match add!(1,2), add!(2,3) etc
            ($a:expr,$b:expr) => {{
                $a + $b
            }};
            // Second arm macth add!(1), add!(2) etc
            ($a:expr) => {{
                $a
            }};
        }

        assert_eq!(add!(1, 2), 3);

        let x = 0;
        assert_eq!(add!(x), 0);
    }

    #[test]
    fn test_macro_rules_add_nonfixed_arguments() {
        // macro_rules! add{
        //     // first arm in case of single argument and last remaining variable/number
        //        ($a:expr)=>{
        //            $a
        //        };
        //    // second arm in case of two arument are passed and stop recursion in case of odd number ofarguments
        //        ($a:expr,$b:expr)=>{
        //            {
        //                $a+$b
        //            }
        //        };
        //    // add the number and the result of remaining arguments
        //        ($a:expr,$($b:tt)*)=>{
        //           {
        //               $a+add!($($b)*)
        //           }
        //        }
        //    }

        macro_rules! add_as{
            (
            // repeated block
          $($a:expr)
            // seperator
           ,
            // zero or more
           *
           )=>{
               {
           // to handle the case without any arguments
           0
           // block to be repeated
           $(-$a)*
             }
            }
        }
        assert_eq!(add_as!(1, 2, 3, 4), 0 - 1 - 2 - 3 - 4);
        // If you look closely, you’ll notice an additional zero is added to
        // the code to make the syntax valid. To remove this zero and make
        // the add expression the same as the argument, we need to create a
        // new macro known as TT muncher.
    }

    #[test]
    fn test_macro_log_syntax() {
        // https://danielkeep.github.io/tlborm/book/mbe-min-debugging.html
        macro_rules! sing {
            () => {};
            ($tt:tt $($rest:tt)*) => {
                log_syntax!($tt "<-token->");
                sing!($($rest)*);
            };
        }

        sing! {
            ^ < @ < . @ *
            '\x08' '{' '"' _ # ' '
            - @ '$' && / _ %
            ! ( '\t' @ | = >
            ; '\x08' '\'' + '$' ? '\x7f'
            , # '"' ~ | ) '\x07'
        }
    }
}
