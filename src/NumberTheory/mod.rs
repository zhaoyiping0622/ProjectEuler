use num::Num;
pub fn get_primes(limit: usize) -> Vec<i32> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();
    is_prime[1] = false;
    for i in 2..=limit {
        if is_prime[i] {
            primes.push(i as i32);
        }
        for j in &primes {
            let j = *j as usize;
            if i * j > limit {
                break;
            }
            is_prime[i * j] = false;
            if i % j == 0 {
                break;
            }
        }
    }
    primes
}
pub fn is_prime<T: Num + PartialOrd + Copy>(x: T) -> bool {
    if x <= T::one() {
        return false;
    }
    let zero = T::zero();
    let one = T::one();
    let mut i = one + one;
    loop {
        if i * i > x {
            return true;
        }
        if x % i == zero {
            return false;
        }
        i = i + one;
    }
}
