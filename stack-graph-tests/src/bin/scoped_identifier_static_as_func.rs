mod powers {
    pub static BOOM: &(dyn Fn() + Sync) = &(|| { println!("boom"); });
}

fn main() {
    powers::BOOM();
    //      ^ defined: 2
}
