pub mod a {
    pub type S = String;

    pub static V: S = S::new();
    //            ^ defined: 2
    //                ^ defined: 2
}

use a::S;
//  ^ defined: 1
//     ^ defined: 2

fn main() {
    let _ = S::new();
    //      ^ defined: 2
}
