fn powMod(a: u128, b: u128) -> u128 {
    let mut a = a;
    let mut b = b;
    let mut ret = 1;
    while b != 0 {
        if b % 2 == 1 {
            ret = ret * a % 10_000_000_000;
        }
        a = a * a % 10_000_000_000;
        b /= 2;
    }
    ret
}
fn main() {
    println!(
        "{}",
        (1..=1000).map(|x| powMod(x, x)).sum::<u128>() % 10_000_000_000
    );
}
