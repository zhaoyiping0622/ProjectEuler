use itertools::Itertools;

fn getNum(nums: &[i32]) -> u64 {
    nums.iter().fold(0, |x, y| x * 10 + (*y) as u64)
}
fn judge(nums: &[i32]) -> bool {
    let primes = [1, 2, 3, 5, 7, 11, 13, 17];
    for i in 0..8 {
        if getNum(&nums[i..i + 3]) % primes[i] != 0 {
            return false;
        }
    }
    true
}
fn main() {
    println!(
        "{}",
        (0..=9)
            .permutations(10)
            .filter(|x| judge(x.clone().as_slice()))
            .map(|x| getNum(x.as_slice()))
            .sum::<u64>()
    );
}
