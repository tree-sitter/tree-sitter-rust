pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;

pub fn f() -> Result<()> {
    //        ^ defined: 2
    Ok(())
}

fn main() {}
