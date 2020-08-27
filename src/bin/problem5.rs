fn gcd(a: i32, b: i32) -> i32 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn test(num: i32) -> Option<i32> {
    for i in 1..=20 {
        if num % i != 0 {
            let t = gcd(i, num);
            return Some(i / t);
        }
    }
    None
}

fn main() {
    let mut ans: i32 = 1;
    loop {
        match test(ans) {
            None => break,
            Some(x) => {
                ans *= x;
            }
        }
    }
    println!("{}", ans);
}
