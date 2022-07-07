#![allow(dead_code, unused_imports)]

mod a {
    pub fn b() {}
    pub mod c {
        pub fn d() {}
    }
}

fn main() {
    use a::{self, b, c::{self, d}};
    //  ^ defined: 3
    //      ^ defined: 3
    //            ^ defined: 4
    //               ^ defined: 5
    //                   ^ defined: 5
    //                         ^ defined: 6

    a::b();
//  ^ defined: 3
    b();
//  ^ defined: 4
    c::d();
//  ^ defined: 5
    d();
//  ^ defined: 6

}
