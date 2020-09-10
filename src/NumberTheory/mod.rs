pub fn getPrimes(limit: usize) -> Vec<i32> {
    let mut isPrime = vec![true; limit + 1];
    let mut primes = Vec::new();
    isPrime[1] = false;
    for i in 2..=limit {
        if isPrime[i] {
            primes.push(i as i32);
        }
        for j in &primes {
            let j = *j as usize;
            if i * j > limit {
                break;
            }
            isPrime[i * j] = false;
            if i % j == 0 {
                break;
            }
        }
    }
    primes
}
