pub struct Point {
    pub x: u32,
    pub y: u32
}

#[allow(non_snake_case)]
pub fn Point(x: u32, y: u32) -> Point {
    //                          ^ defined: 1
    let p = Point { x, y };
    //      ^ defined: 1
    //              ^ defined: 2, 7
    //                 ^ defined: 3, 7
    p
//  ^ defined: 9
}
