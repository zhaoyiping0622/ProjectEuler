use num::{one, BigInt};

fn main() {
    let mut ans: BigInt = one();
    for _i in 0..1000 {
        ans = ans * 2;
    }
    println!(
        "{}",
        ans.to_string()
            .bytes()
            .into_iter()
            .map(|x| (x - '0' as u8) as u64)
            .sum::<u64>()
    )
}
