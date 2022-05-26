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
    //      ^ defined: 1
    //         ^ defined: 2
    //            ^ defined: 3
    //                ^ defined: 4

    let a::b::C { d } = x;
    //  ^ defined: 1
    //     ^ defined: 2
    //        ^ defined: 3
    //            ^ defined: 4

    let ok = d == a::ANSWER;
    //            ^ defined: 1
    //               ^ defined: 8
    assert!(ok);

    let _a: crate::a::b::C =
    //      ^ defined: 1
    //             ^ defined: 1
    //                   ^ defined: 3
        self::a::b::C { d: 8 };
    //  ^ defined: 1
    //        ^ defined: 1
    //              ^ defined: 3
}
