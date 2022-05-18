pub struct Number(String);

impl Number {
    fn add(self, _other: Number) -> Number {
        todo!();
    }
}

pub fn sum(nums: impl IntoIterator<Item = Number>) -> Number {
    let mut nums = nums.into_iter();
    let first = nums.next().unwrap();
    nums.fold(first, |acc, next| acc.add(next))
        //                       ^ defined: 12
        //                               ^ defined: 12
}

fn main() {}
