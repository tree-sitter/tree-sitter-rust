/* --- path: main.rs --- */

use a::b::c::d::E;

mod a;

fn main() {
    E::run();
//  ^ defined: 26
}

/* --- path: a.rs --- */

pub mod b;

/* --- path: a/b.rs --- */

pub mod c;

/* --- path: a/b/c.rs --- */

pub mod d;

/* --- path: a/b/c/d.rs --- */

pub struct E {}

impl E {
    pub fn run() {
        println!("hello world");
    }
}
