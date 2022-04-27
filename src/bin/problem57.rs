use fraction::BigFraction;
use fraction::One;
fn main() {
    let one = BigFraction::one();
    let mut now: BigFraction = one.clone();
    let mut ans = 0;
    for _ in 0..1000 {
        now = one.clone() + one.clone() / (one.clone() + now);
        let a = now.numer().unwrap().clone();
        let b = now.denom().unwrap().clone();
        if a.to_string().len() > b.to_string().len() {
            ans += 1;
        }
    }
    println!("{}", ans);
}
