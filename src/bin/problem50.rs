use ProjectEuler::NumberTheory::get_primes;
fn main() {
    let primes = get_primes(1000_000);
    let mut isPrime = vec![false; 1000_000];
    primes.iter().for_each(|&x| isPrime[x as usize] = true);
    let mut ans = (0, 0);
    for i in 0..primes.len() {
        let mut sum = primes[i];
        let mut count = 1;
        for j in i + 1..primes.len() {
            sum += primes[j];
            count += 1;
            if sum >= 1000_000 {
                break;
            }
            if isPrime[sum as usize] && count > ans.1 {
                ans = (sum, count);
            }
        }
    }
    println!("{:?}", ans);
}
