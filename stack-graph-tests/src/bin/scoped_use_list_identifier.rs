pub mod a {
    pub fn b() {}
}

fn main() {
    use a::{b};
    //  ^ defined: 1
    //      ^ defined: 2
    b();
//  ^ defined: 2
}
