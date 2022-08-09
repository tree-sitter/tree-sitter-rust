/* --- path: src/bin/qualifiers_crate.rs --- */
// The full path matters because this test uses `crate::`.

#![allow(dead_code)]

fn f() {}
pub mod a {
    fn f() {}
    pub mod b {
        fn f() {}

        pub fn test1() {
            use crate::a;
            //         ^ defined: 7
            a::f();
            // ^ defined: 8
        }

        pub fn test2() {
            use crate::{a};
            //          ^ defined: 7
            a::f();
            // ^ defined: 8
        }

        pub fn test3() {
            use crate::{a::{self, b}};
            //          ^ defined: 7
            //              ^ defined: 7
            //                    ^ defined: 9
            a::f();
            // ^ defined: 8
            b::f();
            // ^ defined: 10
        }
    }
}

fn main() {}
