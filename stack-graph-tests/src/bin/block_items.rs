#![allow(dead_code)]
fn test() {
    fn f() {}
    struct Info;

    {
        f();
    //  ^ defined: 11
        let _ = a::N;
        //      ^ defined: 14
        fn f() -> Info { todo!(); }
        //        ^ defined: 13
        struct Info(String);
        mod a { pub const N: usize = 10; }
    }
}

fn main() {}
