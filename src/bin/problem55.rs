use num::BigUint;
fn get_rev(x: &BigUint) -> BigUint {
    let a = format!("{}", x);
    let b = a.chars().rev().collect::<String>().parse();
    if let Ok(x) = b {
        x
    } else {
        panic!("err:{:?}, value:{:?}", b, a)
    }
}

fn run(x: &BigUint) -> BigUint {
    x + get_rev(x)
}
fn is_palindrome(x: &BigUint) -> bool {
    *x == get_rev(x)
}
fn check(mut x: BigUint) -> bool {
    for _ in 0..50 {
        let newx = run(&x);
        if is_palindrome(&newx) {
            return false;
        }
        x = newx;
    }
    true
}
fn main() {
    println!(
        "{}",
        (10..=10_000)
            .filter(|x| check(BigUint::new(vec![*x])))
            .count()
    )
}
