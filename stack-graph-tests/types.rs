pub struct Frob;

pub struct Point(u32, u32);

pub struct A {
    x: u32,
    y: Point,
    // ^ defined: 3
    next: Option<Box<A>>,
    //               ^ defined: 5
}

pub enum MyResult<T, E> {
    Ok(T),
    // ^ defined: 13
    Err {
        err: E,
        //   ^ defined: 13
        blame: String,
        more_issues: Vec<E>,
        //               ^ defined: 13
    },
    Unsure,
}

pub const FROB: Frob = Frob;
//              ^ defined: 1
//                     ^ defined: 1

pub const ORIGIN: Point = Point(0, 0);
//                ^ defined: 3
//                        ^ defined: 3

pub const POINT_FN: fn(u32, u32) -> Point = Point;
//                                  ^ defined: 3
//                                          ^ defined: 3