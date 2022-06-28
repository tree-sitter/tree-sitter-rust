/* --- path: src/bin/scoped_identifier.rs --- */
// The full path matters because this test uses `crate::`.

mod a {
    pub mod b {
        pub struct C {
            pub d: i32,
        }
    }

    pub const ANSWER: i32 = 7;
}

fn main() {
    let x = a::b::C { d: 7 };
    //      ^ defined: 4
    //         ^ defined: 5
    //            ^ defined: 6
    //                ^ defined: 7

    let a::b::C { d } = x;
    //  ^ defined: 4
    //     ^ defined: 5
    //        ^ defined: 6
    //            ^ defined: 7

    let ok = d == a::ANSWER;
    //            ^ defined: 4
    //               ^ defined: 11
    assert!(ok);

    let _a: crate::a::b::C =
    //      ^ defined: 4
    //             ^ defined: 4
    //                   ^ defined: 6
        self::a::b::C { d: 8 };
    //  ^ defined: 4
    //        ^ defined: 4
    //              ^ defined: 6
}
