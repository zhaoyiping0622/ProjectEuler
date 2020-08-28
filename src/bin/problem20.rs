use num::{one, BigInt};
fn main() {
    let mut ans: BigInt = one();
    for i in 1..=100 {
        ans = ans * i;
    }
    println!(
        "{}",
        ans.to_string()
            .bytes()
            .map(|x| (x - '0' as u8) as u64)
            .sum::<u64>()
    );
}
