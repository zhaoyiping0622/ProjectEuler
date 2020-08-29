const LIMIT: usize = 10000;
fn main() {
    let mut isPrime = vec![true; LIMIT];
    let mut primes = Vec::new();
    isPrime[1] = false;
    isPrime[0] = false;
    for i in 2..LIMIT {
        if isPrime[i] {
            primes.push(i);
        }
        for x in &primes {
            if x * i >= LIMIT {
                break;
            }
            isPrime[x * i] = false;
            if x % i == 0 {
                break;
            }
        }
    }
    let mut canWrite = [false; LIMIT];
    canWrite[1] = true;
    for x in primes {
        for j in 0..1000 {
            let t = x + 2 * j * j;
            if t < LIMIT {
                canWrite[t] = true;
            }
        }
    }
    for (index, &x) in canWrite.iter().enumerate() {
        if index % 2 == 1 && !x {
            println!("{}", index);
            return;
        }
    }
    unreachable!();
}
