/* --- path: src/bin/use_self.rs --- */
// The full path matters because this test uses `crate::`.

#![allow(dead_code, unused_imports)]

mod a {
    pub fn b() {}
    pub mod c {
        pub fn d() {}
    }
}

mod e {
    pub fn f() {
        use crate::a::{self, b, c::{self, d}};
        //         ^ defined: 6
        //             ^ defined: 6
        //                   ^ defined: 7
        //                      ^ defined: 8
        //                          ^ defined: 8
        //                                ^ defined: 9

        a::b();
    //  ^ defined: 6
        b();
    //  ^ defined: 7
        c::d();
    //  ^ defined: 8
        d();
    //  ^ defined: 9
    }
}

fn main() {
    use a::{self, b, c::{self, d}};
    //  ^ defined: 6
    //      ^ defined: 6
    //            ^ defined: 7
    //               ^ defined: 8
    //                   ^ defined: 8
    //                         ^ defined: 9

    a::b();
//  ^ defined: 6
    b();
//  ^ defined: 7
    c::d();
//  ^ defined: 8
    d();
//  ^ defined: 9

}
