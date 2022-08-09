// https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html?#destructuring-structs

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    //              ^ defined: 4
    let Point { x: a, y: b } = p;
    //  ^ defined: 3
    //          ^ defined: 4
    //                ^ defined: 5
    assert_eq(0, a);
    //           ^ defined: 11
    assert_eq(7, b);
    //           ^ defined: 11
}

// function because the parser doesn't parse anything in macros :(
#[track_caller]
fn assert_eq(left: i32, right: i32) {
    assert_eq!(left, right);
}
