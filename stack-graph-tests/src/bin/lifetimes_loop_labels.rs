// Example from <https://doc.rust-lang.org/book/ch03-05-control-flow.html>
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
                //    ^ defined: 4
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

pub fn siblings() {
    'label: loop {
        break 'label;
        //    ^ defined: 26
    }
    'label: loop {
        break 'label;
        //    ^ defined: 30
    }
}

// Lifetimes and loop labels look the same, but they're separate namespaces.
pub fn shadowing<'a>(s: &'a str) -> &'a str {
    'a: loop {
        let x: &'a str = s.split_whitespace().next().unwrap();
        //      ^ defined: 37
        break 'a x;
        //    ^ defined: 38
    }
}
