pub enum A {
    M {
        message: String
    }
}

fn ignore(_message: String) {
}

// Resolving the variant name and fields in a pattern: 4 cases
pub fn pattern_qualified_shorthand(x: A) {
    match x {
        A::M {
    //  ^ defined: 1
        // ^ defined: 2
            message
        //  ^ defined: 3
        } => ignore(message),
        //          ^ defined: 16
    }
}

pub fn pattern_qualified_longhand(x: A) {
    match x {
        A::M { message:
        //     ^ defined: 3
               actual
        } => ignore(actual),
        //          ^ defined: 27
    }
}

// When the variant is imported.
pub fn pattern_imported_shorthand(x: A) {
    use A::M;
    //  ^ defined: 1
    //     ^ defined: 2

    match x {
        M {
    //  ^ defined: 2
            message
        //  ^ defined: 3
        } => ignore(message),
        //          ^ defined: 42
    }
}

pub fn pattern_imported_longhand(x: A) {
    use A::M;
    match x {
        M { message:
        //  ^ defined: 3
               actual
        } => ignore(actual),
        //          ^ defined: 54
    }
}

pub fn literals() -> [A; 3] {
    use A::M;
    let message = "ok".to_string();
    [
        A::M { message: String::new() },
        // ^ defined: 2
        //     ^ defined: 3
        M { message: String::new() },
    //  ^ defined: 2
        //  ^ defined: 3
        M { message },
    //  ^ defined: 2
        //  ^ defined: 3, 62
    ]
}

fn main() {}

