#![allow(dead_code)]
fn test() {
    fn f() {}
    struct Info;

    {
        f();
    //  ^ defined: 9
        fn f() -> Info { todo!(); }
        //        ^ defined: 11
        struct Info(String);
    }
}

fn main() {}
