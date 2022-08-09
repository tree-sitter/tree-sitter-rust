pub struct T;

pub const K: usize = 0;

pub struct V<T, const K: usize> {
    pub t: T,
    //     ^ defined: 5
    pub u: Box<[T; K]>,
    //          ^ defined: 5
    //             ^ defined: 5
    pub v: Option<Box<V<T, {K}>>>, // TODO: it should also work without braces
    //                  ^ defined: 5
    //                      ^ defined: 5
}

pub fn f<T>(
    t: T
    // ^ defined: 16
) -> T {
//   ^ defined: 16
    t
}

fn main() {}
