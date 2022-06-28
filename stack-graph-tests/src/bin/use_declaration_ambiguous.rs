/* --- path: src/bin/use_declaration_ambiguous.rs --- */
// The full path matters because this test uses `crate::`.

#![allow(dead_code, unused_imports, non_camel_case_types)]

mod ambig1 {
    /// A module whose name is the same as a crate in the extern prelude.
    mod anyhow {
        pub fn banana() {}
    }

    //use anyhow::banana as b1; // error: `anyhow` is ambiguous
    //use anyhow::Error as e1; // error: `anyhow` is ambiguous
    use self::anyhow::banana; // unambiguously refer to the module
    //                ^ defined: 9
    use ::anyhow::Error; // unambiguously refer to the crate

    fn test() {
        banana();
    //  ^ defined: 9
    }

    fn test2() {
        use crate::ambig1::anyhow::banana; // unambiguously refer to the module
        //                         ^ defined: 9
        banana();
    //  ^ defined: 9
    }
}


mod ambig2 {
    /// A type whose name is the same as a crate in the extern prelude.
    enum anyhow { Banana }

    //use anyhow::Banana as b1; // error: `anyhow` is ambiguous
    //use anyhow::Error as e1; // error: `anyhow` is ambiguous
    use self::anyhow::Banana; // unambiguously refer to the module
    //                ^ defined: 34
    use ::anyhow::Error; // unambiguously refer to the crate

    fn test() {
        let _ = Banana;
        //      ^ defined: 34
    }

    fn test2() {
        use crate::ambig2::anyhow::Banana; // unambiguously refer to the module
        //                         ^ defined: 34
        let _ = Banana;
        //      ^ defined: 34
    }
}

fn main() {}
