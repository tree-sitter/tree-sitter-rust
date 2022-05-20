mod a {
    pub mod b {
        pub struct C {
            pub d: i32,
        }
    }
}

fn main() {
    let _ = a::b::C { d: 7 };
    //      ^ defined: 1
    //         ^ defined: 2
    //            ^ defined: 3
    //                ^ defined: 4
}
