pub fn foo(a: i32) -> i32 {
    let a = a + 1;
    //      ^ defined: 1
    {
        let a = 2 * a;
        //          ^ defined: 2
        let _ = a;
        //      ^ defined: 5
    }
    let a = 1 - a;
    //          ^ defined: 2
    a
//  ^ defined: 10
}

pub fn x() {}

pub fn bar(x: i32) -> i32 {
    x + 1
//  ^ defined: 18
}
