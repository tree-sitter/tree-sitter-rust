pub struct T;

pub struct V<T> {
    pub t: T,
    //     ^ defined: 3
    pub u: Box<T>,
    //         ^ defined: 3
}

pub fn f<T>(
    t: T
    // ^ defined: 10
) -> T {
//   ^ defined: 10
    t
}
