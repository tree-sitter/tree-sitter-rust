/* --- path: a.rs --- */

pub mod b {
    pub fn c() {}
}

/* --- path: lib.rs --- */

mod a;

pub fn working() {
    use a::b::c;
    //     ^ defined: 3
    c();
//  ^ defined: 4
}
