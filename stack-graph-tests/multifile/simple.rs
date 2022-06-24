/*--- path: src/x.rs ---*/

pub const N: i32 = 42;

/*--- path: src/main.rs ---*/

mod x;

use crate::x::N;

fn main() {
    std::process::exit(N);
    //                 ^ defined: 3
}

