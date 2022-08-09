pub struct Parser<'src> {
    pub source: &'src str,
    //           ^ defined: 1
    pub point: usize,
}

pub fn f<'a>(
    table: &'a [(&'a str, Vec<u8>)],
    //      ^ defined: 7
    //            ^ defined: 7
    name: &str
) -> &'a [u8] {
//    ^ defined: 7
    for (tname, tvalue) in table {
        if *tname == name {
            return tvalue;
        }
    }
    &[]
}

impl<'src>
    Parser<'src>
//         ^ defined: 22
{
    pub fn source(&'src self) -> &'src str {
        //         ^ defined: 22
        //                        ^ defined: 22
        self.source
    }
}

fn main() {}
