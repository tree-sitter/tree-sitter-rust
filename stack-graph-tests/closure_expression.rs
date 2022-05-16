struct Number(String);

impl Number {
    fn add(self, other: Number) -> Number {
        todo!();
    }
}

fn sum(nums: impl IntoIterator<Item = Number>) -> Number {
    let mut nums = nums.into_iter();
    let first = nums.next().unwrap();
    nums.fold(first, |acc, next| acc.add(next))
        //                       ^ defined: 12
        //                               ^ defined: 12
}
