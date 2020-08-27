fn solve() -> i32 {
    let mut isPrime = [true; 100_0000];
    let mut primes = Vec::new();
    isPrime[1] = false;
    for i in 1..isPrime.len() {
        if isPrime[i] {
            primes.push(i);
            if primes.len() == 10_001 {
                return *primes.last().unwrap() as i32;
            }
        }
        for &x in primes.iter() {
            if x * i >= isPrime.len() {
                break;
            }
            isPrime[x * i] = false;
            if x % i == 0 {
                break;
            }
        }
    }
    0
}

fn main() {
    println!("{}", solve());
}
