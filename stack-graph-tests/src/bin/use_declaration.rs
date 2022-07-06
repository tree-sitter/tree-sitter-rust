#![allow(dead_code, unused_imports)]

use std::path::Path;

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

fn basic_example() {
    use a::b::c;
    //  ^ defined: 5
    //     ^ defined: 6
    //        ^ defined: 7
    c();
//  ^ defined: 7
}

fn rust_reference_examples() {
    // From <https://doc.rust-lang.org/reference/items/use-declarations.html>.

    fn c() {}
    fn e() {}
    fn f() {}
    fn i() {}

    {
        use a::b::{c, d, e::f, g::h::i};
        //  ^ defined: 5
        //     ^ defined: 6
        //         ^ defined: 7
        //            ^ defined: 8
        //               ^ defined: 13
        //                  ^ defined: 14
        //                     ^ defined: 16
        //                        ^ defined: 17
        //                           ^ defined: 18
        c();
    //  ^ defined: 7
        d::e::f();
    //  ^ defined: 8
        // ^ defined: 9
        //    ^ defined: 10
        f();
    //  ^ defined: 14
        i();
    //  ^ defined: 18
    }

    {
        use a::b::{self, c, d::e};
        //  ^ defined: 5
        //     ^ defined: 6
        //         ^ defined: 6
        //               ^ defined: 7
        //                  ^ defined: 8
        //                     ^ defined: 9
        // TODO - support `self` in use-declarations
        c();
    //  ^ defined: 7
        e::f();
    //  ^ defined: 9
        // ^ defined: 10
    }

    {
        use p::q::r as x;
        //  ^ defined: 24
        //     ^ defined: 25
        //        ^ defined: 26
        x();
    //  ^ defined: 26
    }

    {
        use a::b::{self as ab, c as abc};
        //  ^ defined: 5
        //     ^ defined: 6
        //         ^ defined: 6
        //                     ^ defined: 7
        // TODO - support aliased self import
        abc();
    //  ^ defined: 7
    }

    {
        use a::b::*;
        //  ^ defined: 5
        //     ^ defined: 6
        // TODO - support wildcard imports
    }

    {
        use a::b::{self as ab, c, d::{*, e::f}};
        //  ^ defined: 5
        //     ^ defined: 6
        //         ^ defined: 6
        //                     ^ defined: 7
        //                        ^ defined: 8
        //                               ^ defined: 9
        //                                  ^ defined: 10
        // TODO - support aliased self import
        c();
    //  ^ defined: 7
        // TODO - support wildcard imports
        f();
    //  ^ defined: 10
    }
}

fn main() {}
