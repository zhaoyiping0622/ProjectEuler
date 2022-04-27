fn is_prime(x: u32) -> bool {
    for i in 2.. {
        if i * i > x {
            return true;
        }
        if x % i == 0 {
            return false;
        }
    }
    unreachable!()
}
fn main() {
    // let primes = getPrimes(100_000_000);
    let mut values = Vec::new();
    let mut now = 1;
    let mut cnt = 0;
    // let is_prime = move |x| primes.binary_search(&x).is_ok();
    for i in 1.. {
        for _ in 0..4 {
            now += 2 * i;
            values.push(now);
            if is_prime(now) {
                cnt += 1;
            }
        }
        if cnt * 10 < values.len() + 1 {
            println!("{}", 2 * i + 1);
            break;
        }
    }
}
