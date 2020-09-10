use num::{one, BigInt};

fn main() {
    let mut factor: Vec<BigInt> = Vec::new();
    factor.push(one());
    for i in 1..=100 {
        factor.push(factor.last().unwrap() * BigInt::from(i));
    }
    let mut ans = 0;
    for i in 1..=100 {
        for j in 0..=i {
            let tmp = &factor[i] / &factor[j] / &factor[i - j];
            if tmp > BigInt::from(1000_000) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
