fn main() {
    let mut isPrime = [true; 100_0000];
    let mut primes = Vec::new();
    isPrime[0] = false;
    isPrime[1] = false;
    for i in 2..isPrime.len() {
        if isPrime[i] {
            primes.push(i as i32);
        }
        for &x in primes.iter() {
            if x * i as i32 >= isPrime.len() as i32 {
                break;
            }
            isPrime[x as usize * i] = false;
            if x as usize % i == 0 {
                break;
            }
        }
    }
    let mut now: i64 = 600851475143;
    let mut ans = 0;
    for x in primes {
        while now % x as i64 == 0 {
            ans = x as i64;
            now /= x as i64;
        }
    }
    println!("{}", std::cmp::max(ans, now));
}
