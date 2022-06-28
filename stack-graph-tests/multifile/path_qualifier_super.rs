/* --- path: a.rs --- */

pub mod b;

pub fn f1() {
    super::f();
    //     ^ defined: 33
}

pub fn f2() {
    use super::f;
    f();
//  ^ defined: 33
}

/* --- path: a/b.rs --- */

pub fn f3() {
    super::f1();
    //     ^ defined: 5
}

pub fn f4() {
    use super::f2;
    f2();
//  ^ defined: 10
}

/* --- path: lib.rs --- */

pub mod a;

fn f() {}
