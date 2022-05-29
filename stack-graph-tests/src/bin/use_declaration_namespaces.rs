pub mod a {
    pub const VALUE_ONLY: i32 = -1;
    pub enum TypeOnly { A, B }
    pub type Both = &'static str;
    #[allow(non_upper_case_globals)]
    pub const Both: Both = "Both";
}

pub mod b {
    #[allow(non_camel_case_types)]
    pub struct VALUE_ONLY {}  // a type named VALUE_ONLY
    use crate::a::VALUE_ONLY;  // does not conflict with this import

    pub fn test(_v: VALUE_ONLY) -> i32 { VALUE_ONLY }
    //              ^ defined: 11
    //                                   ^ defined: 2

    #[allow(non_upper_case_globals)]
    static TypeOnly: [u8; 40] = [0; 40];  // a value named TypeOnly
    use crate::a::TypeOnly; // does not conflict with this import

    pub fn test2(_v: TypeOnly) -> usize { TypeOnly.len() }
    //               ^ defined: 3
    //                                    ^ defined: 19

    use crate::a::Both;  // imports both definitions
    pub fn test3() -> Both { Both }
    //                ^ defined: 4
    //                       ^ defined: 6
}

fn main() {}
