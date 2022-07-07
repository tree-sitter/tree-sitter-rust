/* --- path: src/bin/use_crate.rs --- */
// The full path matters because this test uses `crate`.

#![allow(dead_code)]

fn f() {}
pub mod a {
    fn f() { panic!(); }
    pub mod b {
        fn f() { panic!(); }

        pub fn test_crate_root_imports() {
            use crate as main1;
            main1::f();
            //     ^ defined: 6
            use {crate as main2};
            main2::f();
            //     ^ defined: 6
            use crate::{self as main3};
            main3::f();
            //     ^ defined: 6
        }
    }
}

fn main() {}
