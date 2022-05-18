struct Parser<'a> {
    text: &'a str,
    point: usize,
}

fn f(s: &str) {
    let mut _parser = Parser { text: s, point: 0 };
    //                ^ defined: 1
    //                         ^ defined: 2
    //                               ^ defined: 6
    //                                  ^ defined: 3
    todo!();
}
