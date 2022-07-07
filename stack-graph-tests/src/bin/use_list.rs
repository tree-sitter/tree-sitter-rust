#![allow(dead_code, unused_imports)]

mod a {
    pub mod b {
        pub fn c() {}
        pub mod d {
            pub mod e {
                pub fn f() {}
            }
        }
        pub mod e {
            pub fn f() {}
        }
        pub mod g {
            pub mod h {
                pub fn i() {}
            }
        }
    }
}

pub mod p {
    pub mod q {
        pub fn r() {}
    }
}

fn test1() {
    // The `use {a};` here is a hard case.
    // TODO - support redundantly importing a module that's already in scope
    use { a, a::b, a::b::c };
    //       ^ defined: 3
    //          ^ defined: 4
    //             ^ defined: 3
    //                ^ defined: 4
    //                   ^ defined: 5
}

fn test2() {
    use {
        a::b::{
    //  ^ defined: 3
        // ^ defined: 4
            self,
        //  ^ defined: 4
            c,
        //  ^ defined: 5
            d::e::f,
        //  ^ defined: 6
            // ^ defined: 7
            //    ^ defined: 8
            e::f as f2,
        //  ^ defined: 11
            // ^ defined: 12
            g::h::i,
        //  ^ defined: 14
            // ^ defined: 15
            //    ^ defined: 16
        },
        p::{
    //  ^ defined: 22
            self,
        //  ^ defined: 22
            q::{self, r},
        //  ^ defined: 23
            //  ^ defined: 23
            //        ^ defined: 24
        }
    };
}

fn main() {}
