pub struct Point(i32, i32);

pub fn transpose(Point(
    //           ^ defined: 1
    x,
    y
): Point) -> Point {
    Point(y, x)
    //    ^ defined: 6
    //       ^ defined: 5
}

pub fn handle(r: std::io::Result<Point>) {
    match r {
        Err(err) => { let _ = err; }
        //                    ^ defined: 15
        Ok(Point(x, y)) => { let _ = Point(x, y); }
        //                           ^ defined: 1
        //                                 ^ defined: 17
        //                                    ^ defined: 17
    }
}

fn main() {}
