/*--- path: src/pine.rs ---*/

pub mod apple {
    pub const SAUCE: &str = "ponies";
}

pub use crate::butter::FLY;

/*--- path: src/lib.rs ---*/

pub mod pine;

pub mod butter {
    pub static FLY: [u8; 256] = [0; 256];

    pub struct Scotch {}
}

pub fn tests() {
    use crate::butter::Scotch;
    use crate::pine::apple::SAUCE;
    use crate::pine::FLY;

    let _ = Scotch {};
    //      ^ defined: 16
    let _ = SAUCE;
    //      ^ defined: 4
    let _ = FLY;
    //      ^ defined: 14
}
