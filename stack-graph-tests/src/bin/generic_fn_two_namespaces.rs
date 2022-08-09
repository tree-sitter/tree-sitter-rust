// A generic function's type parameters and parameters are two different scopes.
#![allow(non_camel_case_types, dead_code)]

pub trait b {}

pub fn f<a: b>(
//          ^ defined: 4
    b: a
//     ^ defined: 6
) -> a {
//   ^ defined: 6
    b
//  ^ defined: 8
}

// once more, this time in hard mode
pub fn g<a: b>(
//          ^ defined: 4
    a: a
//     ^ defined: 17
) -> a {
//   ^ defined: 17
    a
//  ^ defined: 19
}


fn main() {}
