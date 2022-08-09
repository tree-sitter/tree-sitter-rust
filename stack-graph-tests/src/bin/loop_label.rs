fn read_int() -> Option<i32> { todo!() }

fn main() {
    'outer: while let Some(x) = read_int() {
        if x < 0 {
        // ^ defined: 4
            continue 'outer;
            //       ^ defined: 4
        }
    }
}
