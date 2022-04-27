use num::BigUint;
fn digit_sum(x: BigUint) -> u32 {
    x.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap_or(0))
        .sum()
}
fn main() {
    println!(
        "{}",
        (1..100)
            .map(|x| BigUint::new(vec![x]))
            .map(|x| { (1..100).map(|y| x.pow(y)).map(digit_sum).max().unwrap_or(0) })
            .max()
            .unwrap_or(0)
    )
}
