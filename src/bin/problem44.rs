use num::integer::Roots;
use std::cmp::min;

fn isPentagonal(x: u64) -> bool {
    x.sqrt() * x.sqrt() == x && (1 + x.sqrt()) % 6 == 0
}

fn judge(a: u64, b: u64) -> bool {
    let sum = a + b;
    let diff = {
        if a > b {
            a - b
        } else {
            b - a
        }
    };
    isPentagonal(24 * sum + 1) && isPentagonal(24 * diff + 1)
}

fn main() {
    let mut nums = Vec::new();
    for i in 1..10000 {
        nums.push(i * (3 * i - 1) / 2);
    }
    let mut ans = None;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if judge(nums[i], nums[j]) {
                ans = match ans {
                    None => Some(nums[j] - nums[i]),
                    Some(x) => Some(min(x, nums[j] - nums[i])),
                }
            }
        }
    }
    println!("{:?}", ans.unwrap());
}
