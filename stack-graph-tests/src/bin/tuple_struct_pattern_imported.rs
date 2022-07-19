#![allow(dead_code)]

enum S {
    T(i32),
    U,
}

use S::T;

mod a {
    pub struct T(pub i32);
}

fn main() {
    let s = S::U;
    type T = a::T;
    match s {
        T(_a) => {}
    //  ^ defined: 4
        S::U => {}
    }
}
