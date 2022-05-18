// Don't crash when imports are present.

#![allow(dead_code, unused_imports)]

use std::path::Path;

fn test() {
    // examples from the Rust reference
    { use a::b::{c, d, e::f, g::h::i}; }
    { use a::b::{self, c, d::e}; }
    { use p::q::r as x; }
    { use a::b::{self as ab, c as abc}; }
    { use a::b::*; }
    { use a::b::{self as ab, c, d::{*, e::f}}; }
}

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

fn main() {}
