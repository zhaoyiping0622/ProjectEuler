use num::integer::gcd;

fn judge(numerator: i32, denominator: i32) -> bool {
    let a1 = numerator / 10;
    let a2 = numerator % 10;
    let b1 = denominator / 10;
    let b2 = denominator % 10;
    if a1 == b1 && a1 != 0 {
        a2 * denominator == b2 * numerator
    } else if a1 == b2 && a1 != 0 {
        a2 * denominator == b1 * numerator
    } else if a2 == b1 && a2 != 0 {
        a1 * denominator == b2 * numerator
    } else if a2 == b2 && a2 != 0 {
        a1 * denominator == b1 * numerator
    } else {
        false
    }
}
fn main() {
    let mut a = 1;
    let mut b = 1;
    for i in 10..100 {
        for j in i + 1..100 {
            if judge(i, j) {
                a *= i;
                b *= j;
            }
        }
    }
    println!("{}", b / gcd(a, b));
}
