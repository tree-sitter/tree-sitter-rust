pub enum Color {
    Rgb(u8, u8, u8),
    Infrapink
}

pub fn f(c: Color) -> String {
    match c {
        Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
    //  ^ defined: 1
        //     ^ defined: 2
        Color::Infrapink => "infrapink".to_string(),
    //  ^ defined: 1
        //     ^ defined: 3
    }
}

fn main() {}
