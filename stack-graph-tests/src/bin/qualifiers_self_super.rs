pub fn f() {}
pub mod b {
    pub fn f() {}
    pub mod a {
        pub fn f() {}

        pub fn test1() {
            self::f();
        //  ^ defined: 4
            //    ^ defined: 5
            super::f();
        //  ^ defined: 2
            //     ^ defined: 3
        }

        pub fn test2() {
            self::super::f();
            //    ^ defined: 2
            //           ^ defined: 3
            use self::super::a as a2;
            //  ^ defined: 4
            //        ^ defined: 2
            //               ^ defined: 4
            a2::f();
            //  ^ defined: 5
        }

        pub fn test3() {
            use self::super::super::b as b2;
            //  ^ defined: 4
            //        ^ defined: 2
            //                      ^ defined: 2
            b2::f();
            //  ^ defined: 3
        }

        pub fn test4() {
            super::super::f();
            //            ^ defined: 1
            self::super::super::f();
            //                  ^ defined: 1
        }
    }
}

fn main() {}
