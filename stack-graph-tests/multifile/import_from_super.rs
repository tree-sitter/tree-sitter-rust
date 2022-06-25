/* --- path: src/main.rs --- */

pub mod x;

pub const DEBUG: bool = true;

pub mod y {
    use super::DEBUG;

    pub fn f() {
        if DEBUG {
        // ^ defined: 5
            println!("y::f");
        }
    }
}

/* --- path: src/x.rs --- */

use super::DEBUG;

pub fn f() {
    if DEBUG {
    // ^ defined: 5
        println!("x::f");
    }
}

pub mod z {
    use super::DEBUG;

    pub fn f() {
        if DEBUG {
        // ^ defined: 5
            println!("y::f");
        }
    }
}
