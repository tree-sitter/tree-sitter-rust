enum MyResult<T, E> {
    Fine(T),
    Nope(E),
}

pub fn f() -> MyResult<(), String> {
    MyResult::Fine(())
    //        ^ defined: 2
}

use MyResult::Fine;
//            ^ defined: 2
const ANOTHER: MyResult<&str, &str> = Fine("mess");
//                                    ^ defined: 2

fn main() {}
