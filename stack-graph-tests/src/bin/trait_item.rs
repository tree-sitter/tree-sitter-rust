pub trait Animal {
    const SIZE: usize;
    fn make_noise(&self) {}
}

pub fn f<A: Animal>(henry: A) {
    //      ^ defined: 1
    //                     ^ defined: 6

    // contexts in which we can resolve trait members are limited.
    let _v = Vec::<u8>::with_capacity(<A as Animal>::SIZE);
    //                                 ^ defined: 6
    //                                      ^ defined: 1
    //                                               ^ defined: 2
    Animal::make_noise(&henry);
//  ^ defined: 1
    //      ^ defined: 3
    <A as Animal>::make_noise(&henry);
    //    ^ defined: 1
    //             ^ defined: 3
}

fn main() {}
