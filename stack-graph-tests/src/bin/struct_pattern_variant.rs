pub enum A {
    M {
        message: String
    }
}

pub fn f(x: A) {
    match x {
        A::M { message } => {
    //  ^ defined: 1
        // ^ defined: 2
            // ^ defined: 3
            let _ = message;
        }
    }
}

fn main() {}
