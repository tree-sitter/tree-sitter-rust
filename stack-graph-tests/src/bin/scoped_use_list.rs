mod a {
    pub struct X {}
    pub struct Y {}
}

use a::{X, Y};
//      ^ defined: 2
//         ^ defined: 3

fn main() {
    let _x = X {};
    //       ^ defined: 2
    let _y = Y {};
    //       ^ defined: 3
}
