pub fn is_ok(t: i32) -> bool {
    match t.count_ones() {
        ones@(8..=30) =>
            ones % 2 == 0,
        //  ^ defined: 3
        _ => false,
    }
}

fn main() {}
